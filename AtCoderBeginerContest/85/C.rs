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
    let n:u64 = r.reads();
    let y:u64 = r.readl();

    let min_thousand = (y % 5000) / 1000;
    let mut res = vec![0,0,0];
    if(y % 10000 >= 5000) {
        let rest_ten_thousand = (y - y % 10000) / 10000;
        for i in 0..(rest_ten_thousand + 1) {
            let times_use_five_thousand = min_thousand + 1;
            let times_not_use_five_thousand = min_thousand + 5;
            let rest = (y - y % 10000) - i * 10000;
            for j in 0..(rest / 5000 + 1) {
                if(i + j + times_not_use_five_thousand + (rest - j * 5000) / 1000 == n){
                    res = vec![i,j,5 + (rest - j * 5000) / 1000 + min_thousand];
                    break;
                }
                if(i + j + times_use_five_thousand + (rest - j * 5000) / 1000 == n){
                    res = vec![i,j + 1,(rest - j * 5000) / 1000 + min_thousand];
                    break;
                }
            }

        }
    }else{
        let rest_ten_thousand = (y - y % 10000) / 10000;
        for i in 0..(rest_ten_thousand + 1) {
            let rest = (y - y % 10000) - i * 10000;
            for j in 0..(rest / 5000 + 1) {
                if(i + j + min_thousand + (rest - j * 5000) / 1000 == n){
                    res = vec![i,j,(rest - j * 5000) / 1000 + min_thousand];
                    break;
                }
            }

        }

    }


    if(res[0] == 0 && res[1] == 0 && res[2] == 0){
        println!("{} {} {}", -1, -1, -1);
    }else{
        println!("{} {} {}",res[0], res[1], res[2]);
    }

}
