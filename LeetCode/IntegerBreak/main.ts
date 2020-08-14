const integerBreak: (n: number) => number = (n) => {
  if (n === 2) {
    return 1;
  }
  if (n === 3) {
    return 2;
  }

  if (n % 3 === 0) {
    return 3 ** (n / 3);
  }
  if (n % 3 === 1) {
    return 3 ** ((n - 4) / 3) * 4;
  }

  return 3 ** ((n - 2) / 3) * 2;
};

console.log(integerBreak(8));
console.log(integerBreak(10));
