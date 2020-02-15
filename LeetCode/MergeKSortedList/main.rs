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

pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn vec_to_list(mut value: Vec<i32>) -> Option<Box<ListNode>> {
    match value.pop() {
        Some(x) => Some(Box::new(ListNode {
            val: x,
            next: vec_to_list(value),
        })),
        None => None,
    }
}

pub fn list_to_vec(mut value: Option<Box<ListNode>>, mut v_cup: Vec<i32>) -> Vec<i32> {
    match value {
        Some(x) => {
            v_cup.push(x.val);
            list_to_vec(x.next, v_cup)
        }
        None => v_cup,
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut ans = Vec::new();
    for l in lists {
        let vec = list_to_vec(l,Vec::new());
        for i in vec {
            ans.push(i);
        }
    }
    ans.sort();
    ans.reverse();
    vec_to_list(ans)
}

fn main(){
    // let stdin = stdin();
    // let mut r = utils::StdinReader::new(stdin.lock());
    let mut tar = Vec::new();
    tar.push(vec_to_list(vec![1,4,5]));
    tar.push(vec_to_list(vec![1,3,4]));
    tar.push(vec_to_list(vec![2,6]));
    let ans_vec = list_to_vec(merge_k_lists(tar),Vec::new());
    for i in ans_vec {
        println!("{}",i);
    }
}
