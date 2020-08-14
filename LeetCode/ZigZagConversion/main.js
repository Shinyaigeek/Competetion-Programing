function convert(s, numRows) {
  const l = s.length;
  let i = 0;
  if(numRows === 1) {
    return s
  }
  return Array.from(new Array(l)).map(() => {
    let j = i;
    if (i % (numRows * 2 - 2) === 0) {
      if (i + (numRows * 2 - 2) >= l) {
        if(numRows === 1) {
          i += 1;
        }else{
          i = 1;
        }
      }else{

        i += numRows * 2 - 2;

      }

      return s[j];
    }

    if (i % (numRows * 2 - 2) === numRows - 1) {
      i += numRows * 2 - 2;
      return s[j];
    }

    const r = i % (numRows * 2 - 2);

    if (r < numRows - 1) {
      if (i + (numRows * 2 - 2 - r * 2) >= l) {
        i = r + 1;
      }else{

        i += (numRows * 2 - 2) - r * 2;

      }
      return s[j];
    } else {
      if (i + ((numRows * 2 - 2) - r) * 2 >= l) {
        i = numRows * 2 - 2 - r + 1;
      }else{
        i += ((numRows * 2 - 2) - r) * 2
      }
      return s[j];
    }
  }).join("");
}

console.log(convert("PAYPALISHIRING", 3));
console.log(convert("AB", 1));
