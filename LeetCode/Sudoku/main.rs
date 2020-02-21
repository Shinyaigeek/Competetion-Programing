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

pub fn sudoku(board: &mut Vec<Vec<char>>) {

}

pub fn get_row(board: Vec<Vec<char>>, row: usize) -> Vec<char> {
    let mut res = Vec::new();
    for col in board {
        res.push(col[row]);
    }
    res
}

pub fn check_row(board: Vec<Vec<char>>, row: usize) -> Vec<char> {
    let mut res = Vec::new();
    let target_row = get_row(board, row);
    for i in target_row {
        if( i != '.'){
            res.push(i);
        }
    }
    res
}

pub fn check_column(board: Vec<Vec<char>>, column: usize) -> Vec<char> {
    let mut res = Vec::new();
    for i in 0..9  {
        if( board[column][i] != '.'){
            res.push(board[column][i]);
        }
    }
    res
}

pub fn get_room(board: Vec<Vec<char>>, room:usize) -> Vec<char> {
    let h = room % 3;  //0,1,2
    let v = (room - h) / 3;  //0,1,2

    let mut target_columns = Vec::new();
    for c in 0..9 {
        if (v * 3 <= c) && ((v + 1) * 3 > c) {
            target_columns.push(board[c]);
        }
    }

    let mut res = Vec::new();
    for col in target_columns {
        for i in 0..9 {
            if (h * 3 <= i) && ((h + 1) * 3 > i) {
                res.push(col[i]);
            }  
        }
    }
    res
}

pub fn check_room(board: Vec<Vec<char>>, room:usize) -> Vec<char> {
    let target = get_room(board, room);
    let mut res = Vec::new();
    for i in target { 
        if( i != '.'){
            res.push(i);
        }
    }
    res
}