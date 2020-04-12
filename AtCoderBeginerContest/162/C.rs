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
    let k:usize = r.readl();
    
    let mut res:usize = 0;

    for x in 1..(k + 1) {
        for y in 1..(k + 1) {
            for z in 1..(k+1) {
                res += getGCD(x, y, z)
            }
        }
    } 

    println!("{}", res);
}

pub fn getGCD(aa:usize, bb:usize, cc:usize) -> usize {
    let mut abGcd = 0;
    if(aa > bb) {
        abGcd = gcd(aa, bb)
    }else{
        abGcd = gcd(bb, aa)
    }
    if(abGcd > cc) {
        return gcd(abGcd, cc);
    }else{
        return gcd(cc, abGcd);
    }
}

pub fn gcd(left: usize, right:usize) -> usize {
    if(left % right == 0) {
        return right
    }else{
        return gcd(right, left%right)
    }
}
