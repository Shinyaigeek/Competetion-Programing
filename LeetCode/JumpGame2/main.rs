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
}

fn solve(nums:Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut range = nums[0] + 1;
    let limit = nums.len() as i32; 
    if (limit == 1) {
        return 0;
    }
    if (range >= limit) {
        return 1;
    }
    loop {
        range = step(get_range(&nums, 0, range as usize));
        ans += 1;
        if(range >= limit){
            break
        }
    }
    ans + 1
}

fn get_range(target: &Vec<i32>, start: usize, end: usize) -> Vec<i32> {
    let mut ans = Vec::new();
    for i in start..end {
        ans.push(target[i]);
    }
    ans
}

fn step(target:Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..target.len() {
        if (res < (i as i32) + 1 + target[i]) {
            res = (i as  i32) + 1 + target[i];
        }
    }
    res
}
