use std::vec::Vec;

fn int_to_roman(num: i32) -> String {
    let mut ans:String= String::from("");
    let numStr:String = num.to_string().chars().rev().collect::<String>();
    let mut rank:usize = 0;
    let ten:i32 = 10;
    let romans:Vec<char> = vec!['I','V','X','L','C','D','M'];
    for i in numStr.chars() {
        let tar = i as i32 - 48;
        if tar <= 3 {
            for j in 0..tar {
                ans.push(romans[2 * rank]);
            }
        }else if tar == 4 {
            ans.push(romans[2 * rank + 1]);
            ans.push(romans[2 * rank]);
        }else if tar <= 8{
            for j in 0..(tar - 5) {
                ans.push(romans[2 * rank]);
            }
            ans.push(romans[2 * rank + 1]);
        }else{
            ans.push(romans[2 * rank + 2]);
            ans.push(romans[2 * rank]);
        }
        rank += 1;
    }
    let mut ans = ans.chars().rev().collect::<String>();
    return ans
}

fn main(){
    let test = 1994;
    let ans = int_to_roman(test);
    println!("{}",ans);
}