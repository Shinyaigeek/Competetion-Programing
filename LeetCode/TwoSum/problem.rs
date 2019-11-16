use std::vec::Vec;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         println!("target:{}",target);
//     }
// }

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans:Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len(){
            if(nums[i] + nums[j] == target){
                ans.push(i as i32);
                ans.push(j as i32);
                break
            }
        }
    }
    return ans
}

fn main(){
    let nums = vec![2,7,11,15];
    let target = 9;

    let ans = two_sum(nums,target);
    println!("{},{}",ans[0],ans[1]);
}