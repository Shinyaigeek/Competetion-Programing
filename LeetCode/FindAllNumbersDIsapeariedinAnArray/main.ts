function findDisappearedNumbers(nums: number[]): number[] {
    const res: number[] = [];
    nums.forEach((num, idx) => {
        if(!nums.includes(idx + 1)) {
            res.push(idx + 1)
        }
    })
    return res
};

console.log(findDisappearedNumbers([4,3,2,7,8,2,3,1]))