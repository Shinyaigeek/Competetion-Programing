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
    let n:String = r.readl();
    let k:usize = r.readl();
    let mut dp = vec![vec![[0;2];k + 1];n.len() + 1];
    //  init
    dp[0][0][1] = 1;
    let mut n_chars = n.chars();
    for i in 1..(n.len() + 1) {
        let i_char = match n_chars.next() {
            None => continue,
            Some(c) => c
        };
        let i_num = i_char as i32 - 48;
        for j in 0..(k + 1) {
            if(j == 0){
                dp[i][j] = [1,0];
            }else{
                if(i_num > 0){
                    dp[i][j] = [
                        dp[i-1][j-1][0] * 9 + dp[i-1][j-1][1] * (i_num - 1) + dp[i-1][j][0] + dp[i-1][j][1],
                        dp[i-1][j-1][1]
                    ];

                }else{
                    dp[i][j] = [
                        dp[i-1][j-1][0] * 9 + dp[i-1][j][0],
                        dp[i-1][j][1]
                    ];
                }
            }

        }
    }
    let mut res = 0;
    // count
    // for i in 0..(n.len() + 1) {
    //     for j in 0..(k + 1) {
    //         print!("[0:{},1:{}], ",dp[i][j][0], dp[i][j][1]);
    //     }
    //     println!("");
    // }
    println!("{}",dp[n.len()][k][0] + dp[n.len()][k][1]);
}
