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

    let s: String = r.readl();
    let num_rows = r.readl();

    let ans = split(s, num_rows);

    println!("{}", ans);

}

fn split(s: String, num_rows: i32) -> Vec<Vec<String>> {
    let wave_height = num_rows;
    let wave_period = num_rows - 1;
    let particle:Vec<&str> = s.split("").collect();

    let particle_period = wave_height + wave_period - 1;

    let mut idx:i32 = -1;
    let max: i32 = s.len();

    let mut res:Vec<Vec<String>> = Vec::new();

    let mut res_row = Vec::new();

    for (p) in particle {
        if(idx == -1) || (idx == max) {
            // do nothing
        }else{
            println!("{}: {}", idx, p);

            let rest: i32 = idx % particle_period;

            if(rest < wave_height) {
                res_row.push(p);
                if(rest == wave_height - 1) {
                    res.push(res_row);
                    res_row = Vec::new();
                }
            }else{

            }
        }
        idx += 1;
    }
    
    return res;
}
