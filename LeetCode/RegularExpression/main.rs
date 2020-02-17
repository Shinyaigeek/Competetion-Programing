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
    println!("{}",regularExpression(String::from("aab"), String::from("c*a*b")));
}

fn regularExpression(s:String, p:String) -> bool {
    let mut ps: Vec<&str> = p.split("*").collect();

    struct Parsed {
        un_omitted: String,
        omitted: String,
        next_un_omitted: String
    }

    impl Parsed {
        pub fn new(target: String, is_last:bool) -> Parsed {
            let mut copied = String::from(&target);
            let mut un_omitted = String::from("");
            let mut omitted = String::from(&copied);
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

    if(ps.len() != 1){
        for i in 0..ps.len() {
            let j = ps[i];
            let is_last = i == ps.len() - 1;
            parsed_s.push(Parsed::new(String::from(j),is_last));
        }
    }else {
        let yake = Parsed { un_omitted: String::from(ps[0]), omitted: String::from(""), next_un_omitted : String::from("")};
        parsed_s.push(yake);
    }

    let mut s_copied = s;

    for i in 0..parsed_s.len() {
        println!("-----------start-----------");
        let parsed = &parsed_s[i];
        println!("unommited:{}\nommited: {}",parsed.un_omitted,parsed.omitted);
        if(i == parsed_s.len() - 1 && parsed.omitted.len() + parsed.un_omitted.len() == 0){
            return true;
        }
        if(!dot_starts_with(&s_copied , &parsed.un_omitted)){
            return false;
        }
        let (f,l) = s_copied.split_at(parsed.un_omitted.len());
        println!("l:{}\nf:{}",l,f);
        println!("i:{}\nparsed_s.len:{}",i,parsed_s.len());
        // println!("aa:{}",parsed_s[i + 1].un_omitted);
        if(i < parsed_s.len() - 1){
            let (must_check, next_s) = match dot_find(&l, &parsed_s[i + 1].un_omitted) {
                None => return false,
                Some(s) => l.split_at(s)
            };
            println!("must_check:{}\nnext_s:{}",must_check,next_s);
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
                    // println!("m_c:{}\n",m_c);
                    if(parsed.omitted != "."){
                        if(m_c.to_string() != parsed.omitted){
                            return false;
                        }
                    }
                }
            }
        }

        println!("-----------end-----------");
    }

    true
}

fn dot_starts_with (target:&str, q:&str) -> bool {
    let qo = q.to_string();
    let to = q.to_string();
    let qs: Vec<&str> = qo.split("").collect();
    let ts: Vec<&str> = to.split("").collect();
    for i in 0..ts.len() {
        if (qs[i] != ts[i]) && qs[i] != "." {
            return false;
        }
    }

    true
}

fn dot_find (target:&str, q:&str) -> Option<usize> {
    let to = target.to_string();
    let qo = q.to_string();
    let ts: Vec<&str> = to.split("").collect();
    let qs: Vec<&str> = qo.split("").collect();

    println!("target:{}\nq:{}",target,q);

    if(q == "") && target == "" {
        return Some(0);
    }

    if(q == ""){
        return Some(0);
    }

    if(target == ""){
        return None;
    }

    let mut qi = 1;
    for i in 1..ts.len() - 1 {
        println!("ts[i]:{}\nqs[qi]:{}",ts[i],qs[qi]);
        if(ts[i] == qs[qi]) || qs[qi] == "." {
            qi += 1;
            println!("qi:{}\nqs:{}\ni:{}",qi,qs.len(),i);
            if(qi >= qs.len() - 1) {
                println!("ans i:{}",i - 1);
                return Some(i - 1);
            }
        }else{
            qi = 1;
        }
    }
    None
}
