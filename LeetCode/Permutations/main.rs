pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();

    for i in 0..len {

    }
}

pub fn calc_permute(target: Vec<i32>) ->Vec<Vec<i32>> {
    if(target.len() == 1) {
        return 
    }
    let mut pops:Vec<i32> = vec![];
    for i in
}

pub fn concat_permutes(pops:Vec<i32>, permutes:Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res:Vec<Vec<i32>> = Vec::new();
    for p in pops {
        for per in permutes {
            res.push(vec![vec![p],per].concat());
        }
    }
    res
}

