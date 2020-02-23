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
    let n:i32 = r.reads();
    let k:usize = r.readl();
    let ps:Vec<f64> = r.read_vec();
    let mut p_max = 0;
    let mut es_p:Vec<f64> = vec![];
    for i in ps {
        es_p.push((1_f64 + i) / 2_f64);
    }
    let mut res = 0_f64;
    let mut e_k = 0_f64;
    for i in 0..(es_p.len() - k + 1) {
        if(i == 0){
            for e in i..(i + k) {
                e_k += es_p[e];
            }
        }else{
            e_k -= es_p[i-1];
            e_k += es_p[i + k - 1];
        }
        if(e_k > res){
            res = e_k;
        }
    }
    println!("{}",res);

}
