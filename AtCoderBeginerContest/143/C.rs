use std::io::stdin;

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
        // 区切り文字：スペース
        pub fn reads<T: FromStr>(&mut self) -> T {
            self.read_until(b' ')
        }

        #[allow(unused)]
        #[inline]
        // 区切り文字：改行
        pub fn readl<T: FromStr>(&mut self) -> T {
            self.read_until(b'\n')
        }

        #[inline]
        pub fn read_until<T: FromStr>(&mut self, delim: u8) -> T {
            // self.bufに次のトークンをセットする
            loop {
                self.buf.clear();
                let len = self.reader
                    .read_until(delim, &mut self.buf)
                    .expect("failed reading bytes");
                match len {
                    0 => panic!("early eof"),
                    1 if self.buf[0] == delim => (), // 区切り文字だけなのでもう一度ループ
                    _ => {
                        // トークンが得られた
                        // 最後の文字が区切り文字なら削除
                        if self.buf[len - 1] == delim {
                            self.buf.truncate(len - 1);
                        }
                        break; // ループから脱出
                    }
                }
            }

            // 文字列をT型へパースする
            let elem = unsafe { str::from_utf8_unchecked(&self.buf) };
            elem.parse().unwrap_or_else(|_| panic!(format!("failed parsing: {}", elem)))
        }
    }

}

fn main(){
    let stdin = stdin();
    let mut r = utils::StdinReader::new(stdin.lock());
    let n:usize = r.readl();
    let s:String = r.readl();
    let mut ans = 0;
    let mut previous_color:char = 'D';
    for el in s.chars() {
        if previous_color != el{
            ans += 1;
            previous_color = el;
        }
    }
    println!("{}",ans);
}