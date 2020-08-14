const maximalSquare = (matrix: ("0" | "1")[][]) => {
  if (matrix.length === 0) {
    return 0;
  }
  let len = 0;
  while (isThereSquare(matrix, len + 1)) {
    len += 1;
  }

  return len ** 2;
};

const isThereSquare = (matrix: ("0" | "1")[][], length: number) => {
  const width = matrix[0].length;
  const height = matrix.length;

  for (let y = 0; y < height - length + 1; y++) {
    for (let x = 0; x < width - length + 1; x++) {
      const col: string[] = new Array(length).fill(0);
      const arr: string[][] = new Array(length).fill(col);
      if (
        isSquare(
          arr.map((c, dy) => {
            return c.map((_, dx) => {
              return matrix[dy + y][dx + x];
            });
          })
        )
      ) {
        return true;
      }
    }
  }
};

const isSquare = (matrix: ("0" | "1")[][]) => {
  for (let col of matrix) {
    for (let el of col) {
      if (el === "0") {
        return false;
      }
    }
  }

  return true;
};

console.log(
  maximalSquare([
    ["1", "0", "1", "0", "0"],
    ["1", "0", "1", "1", "1"],
    ["1", "1", "1", "1", "1"],
    ["1", "0", "0", "1", "0"],
  ])
);

console.log(maximalSquare([]));
