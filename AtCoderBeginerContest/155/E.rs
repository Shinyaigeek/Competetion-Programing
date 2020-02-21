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

pub fn min(left: i32, right: i32) -> i32 {
    if(left > right) {
        return right;
    }
    left
}


fn main(){
    let stdin = stdin();
    let mut r = utils::StdinReader::new(stdin.lock());
    let n:String = r.readl();
    let each_digits_of_n:Vec<&str> = n.rsplit_terminator("").collect();
    let mut dp = vec![vec![std::i32::MAX;2]; each_digits_of_n.len()];
    dp[0][0] = 0;
    dp[0][1] = 1;
    for i in 0..each_digits_of_n.len() - 1 {
        let d:i32 = match each_digits_of_n[i].parse() {
            Ok(j) => j,
            Err(e) => continue
        };
        dp[i + 1][0] = min(dp[i][0] + d, dp[i][1] + 10 - d);
        dp[i + 1][1] = min(dp[i][0] + d + 1, dp[i][1] + 10 - d - 1);
    }
    println!("{}",dp[each_digits_of_n.len() - 1][0]);
}
    
