use std::io::stdin;
use std::vec::Vec;
use std::num::Wrapping;

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

const MOD:i64 = 1000000007;


fn main(){
    let stdin = stdin();
    let mut r = utils::StdinReader::new(stdin.lock());
    let n:i32 = r.readl();
    let As:Vec<i64> = r.read_vec();
    let mut ans:i64 = 0;
    let mut max:i64 = 0;
    for a in As.clone() {
        if(max == 0){
            max = a;
            // ans = a;
        }else{
            if(max % a == 0){
                // ans += max / a;
            }else{
                let mut gcm;
                if(max > a){
                    gcm = getGCM(max,a);
                }else{
                    gcm = getGCM(a,max);
                }
                let nannbai = a / gcm;
                max = nannbai * max;
            // println!("gcm:{},a:{}",gcm,a);
            }
        }
        // println!("max:{},a:{}",max,a);
    }
    for a in As {
        ans += max / a % MOD;
    }
    println!("ans:{},max:{}",ans % MOD,max);
}

fn getGCM(base:i64,tar:i64) -> i64 {
    let mut ans:i64 = 1;
    let mut tar_b = tar;
    let mut tar_a = base;
    while(true){
        if(tar_a % tar_b == 0){
            ans = tar_b;
            break
        }
        let temp = tar_b;
        tar_b = tar_a % tar_b;
        tar_a = temp;
    }
    return ans;
}
