use std::io::stdin;
use std::vec::Vec;

mod utils {
    use std::io::BufRead;
    use std::str::{self, FromStr};

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(reader: R) -> StdinReader<R> {
            StdinReader {
                reader: reader,
                buf: Vec::new(),
            }
        }

        #[allow(unused)]
        #[inline]
        pub fn reads<T: FromStr>(&mut self) -> T {
            self.read_until(b' ')
        }

        #[allow(unused)]
        #[inline]
        pub fn readl<T: FromStr>(&mut self) -> T {
            self.read_until(b'\n')
        }

        #[allow(unused)]
        #[inline]
        pub fn read_vec<T: FromStr>(&mut self) -> Vec<T> {
            return self.read_until::<String>(b'\n').trim().split_whitespace().map(|c| c.parse().ok().unwrap()).collect();
        }

        #[inline]
        pub fn read_until<T: FromStr>(&mut self, delim: u8) -> T {
            loop {
                self.buf.clear();
                let len = self.reader
                    .read_until(delim, &mut self.buf)
                    .expect("failed reading bytes");
                match len {
                    0 => panic!("early eof"),
                    1 if self.buf[0] == delim => (),
                    _ => {
                        if self.buf[len - 1] == delim {
                            self.buf.truncate(len - 1);
                        }
                        break;
                    }
                }
            }

            let elem = unsafe { str::from_utf8_unchecked(&self.buf) };
            elem.parse().unwrap_or_else(|_| panic!(format!("failed parsing: {}", elem)))
         }
    }
}


fn main(){
    let stdin = stdin();
    let mut r = utils::StdinReader::new(stdin.lock());

    let n:usize = r.readl();
    let s:String = r.readl();
    let s_splitted:Vec<&str> = s.split("").collect();
    let mut res = 0;

    for x in 0..(n - 2) {
        let s_x = s_splitted.get(x + 1);
        for y in (x + 1)..(n - 1) {
            let s_y = s_splitted.get(y + 1);
            if(s_x != s_y){
                for z in (y + 1)..n {
                    // println!("{}, {}, {}",x,y,z);
                    if(z - y) != (y - x) {
                        let s_z = s_splitted.get(z + 1);
                        if(s_x != s_z) && (s_y != s_z) {
                            // println!("{}, {}, {}",s_x, s_splitted[y + 1], s_splitted[z + 1]);
                            res += 1;
                        }
                    }
                }

            }
        }
    }

    println!("{}", res);
}
