fn my_pow(x: f64, n: i32) -> f64 {
    let mut ans:f64 = 1.0000000;
    if(n == 0){
        // do nothing
    }else if n > 0{
        for i in 0..n{
            ans *= x;
        }
    }else {
        for i in 0..(-1 * n){
            ans /= x;
        }
    }
    return ans
}

fn main() {
    let ans:f64 = my_pow(2.00000, 10);
    println!("{}",ans);
}