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
    println!("{}",regularExpression(String::from("mississippi"), String::from("c*a*b")));
}

fn regularExpression(s:String, p:String) -> bool {
    let ps: Vec<&str> = p.split("*").collect();

    struct Parsed {
        un_omitted: String,
        omitted: String,
        next_un_omitted: String
    }

    impl Parsed {
        pub fn new(target: String, is_last:bool) -> Parsed {
            let mut copied = String::from(&target);
            let mut un_omitted = String::from(&copied);
            let mut omitted = String::from("");
            if(!is_last){
                omitted = match copied.pop() {
                    None => String::from(""),
                    Some(c) => c.to_string()
                };
                un_omitted = copied;
            }
            Parsed { un_omitted: un_omitted, omitted: omitted, next_un_omitted: String::from("")}
        }
    }

    let mut parsed_s:Vec<Parsed>  = Vec::new();

    for i in 0..ps.len() {
        let j = ps[i];
        let is_last = i == ps.len() - 1;
        parsed_s.push(Parsed::new(String::from(j),is_last));
    }

    let mut s_copied = s;

    for i in 0..parsed_s.len() {
        println!("-----------start-----------");
        let parsed = &parsed_s[i];
        println!("unommited:{}\nommited: {}",parsed.un_omitted,parsed.omitted);
        if(i == parsed_s.len() - 1 && parsed.omitted.len() + parsed.un_omitted.len() == 0){
            return true;
        }
        if(!s_copied.starts_with(&parsed.un_omitted)){
            return false;
        }
        let (f,l) = s_copied.split_at(parsed.un_omitted.len());
        println!("l:{}\nf:{}",l,f);
        println!("i:{}\nparsed_s.len:{}",i,parsed_s.len());
        if(i < parsed_s.len() - 1){
            let (must_check, next_s) = match l.find(&parsed_s[i + 1].un_omitted) {
                None => return false,
                Some(s) => l.split_at(s)
            };
            if(must_check.len() != 0) {
                for m_c in must_check.chars() {
                    if(m_c.to_string() != parsed.omitted){
                        return false;
                    }
                }
            }
            s_copied = next_s.to_string();
        }else {
            if(l.len() != 0) {
                for m_c in l.chars() {
                    println!("m_c:{}\n",m_c);
                    if(m_c.to_string() != parsed.omitted){
                        return false;
                    }
                }
            }
        }

        println!("-----------end-----------");
    }

    true
}
