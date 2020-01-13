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
    let words:Vec<String> = r.read_vec();
    let up = "qwertyuiop".split("").map(|s| s.to_string()).collect::<Vec<String>>();
    let middle = "asdfghjkl".split("").map(|s| s.to_string()).collect::<Vec<String>>();
    let bottom = "zxcvbnm".split("").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut res:Vec<String> = Vec::new();
    for word in words {
        let mut tar = "non";
        let mut ans = true;
        for letter in word.split("") {
            if(letter == ""){

            }else if(includes(&up,letter.to_string())){
                if(tar == "middle" || tar == "bottom"){
                    ans = false;
                }
                tar = "up";
            }else if(includes(&middle,letter.to_string())){
                if(tar == "up" || tar == "bottom"){
                    ans = false;
                }
                tar = "middle"
            }else if(includes(&bottom,letter.to_string())) {
                if(tar == "middle" || tar == "up"){
                    ans = false;
                }
                tar = "bottom"
            }else{
                ans = false;
            }
        }
        if(ans){
            res.push(word);
        }
    }
    return res;
}

fn includes(tar:&Vec<String>,base:String) -> bool {
    for s in tar {
        if(s == &base.to_lowercase()){
            return true;
        }
    }
    return false;
}
