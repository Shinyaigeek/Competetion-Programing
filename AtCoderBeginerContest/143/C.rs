use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T{
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main(){
    let n:i32 = read();
    let s:String = read();

    let mut ans = 0;
    let mut previous_color = "";
    for i in 0..n as usize {
        if !(previous_color == &s[..i]){
            ans += 1;
        }
        previous_color = &s[..i];
    }
    println!("{}",ans);
}