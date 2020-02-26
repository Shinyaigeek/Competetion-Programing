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

pub fn get_min(target: Vec<Option<usize>>,from: usize, to:usize) -> usize {
    let mut res = std::usize::MAX;
    for i in from..to {
        let x = match target[i] {
            Some(q) => q,
            None => continue
        };
        if(x < res) {
            res = x;
        }
    }
    res
}

pub fn max(target:&Vec<usize>) -> usize {
    let mut res = 0;
    for &i in target {
        if (res < i){
            res = i;
        }
    }
    res
}

pub fn min(left: usize, right: usize) -> usize {
    if(left > right) {
        return right;
    }
    left
}

fn main(){
    let stdin = stdin();
    let mut r = utils::StdinReader::new(stdin.lock());
    let h:usize = r.reads();
    let n:usize = r.readl();
    let mut a:Vec<usize> = Vec::with_capacity(n);
    let mut b:Vec<usize> = Vec::with_capacity(n);

    for t in 0..n {
        a.push(r.reads());
        b.push(r.readl());
    }


    let damage_max = max(&a);

    // dp[i] is the minimum cost to cause i damages.
    let mut dp = vec![None;1 + h + damage_max];

    // initial
    dp[0] = Some(0);

    // recurrence formula for dp
    for i in 0..n {
        let ai:usize = a[i];
        let bi:usize = b[i];
        for j in 0..dp.len() {
            if(j >= ai) {
                dp[j] = match dp[j] {
                    Some(x) => match dp[j - ai] {
                        Some(y) => Some(min(y + bi, x)),
                        None => Some(x)
                    },
                    None => match dp[j - ai] {
                        Some(y) => Some(y + bi),
                        None => None
                    }
                }
            }
        }
    }

    println!("{}", get_min(dp, h, h + 1 + damage_max));

}
