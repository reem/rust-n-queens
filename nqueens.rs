extern crate test;

use std::vec_ng::Vec;

fn main() {
  println!("{0}", nQueens(11));
}

fn nQueens(n: int) -> int {
  return nQueensHelper((1 << n) - 1, 0, 0, 0);
}

fn nQueensHelper(magicQ: int, leftDiags: int, columns: int, rightDiags: int) -> int {
  let mut solutions = 0;
  let mut validSpots = !(leftDiags | columns | rightDiags) & magicQ;
  while validSpots != 0 {
    let spot = -validSpots & validSpots;
    validSpots = validSpots ^ spot;
    solutions += nQueensHelper(
      magicQ,
      (leftDiags | spot) << 1,
      (columns | spot),
      (rightDiags | spot) >> 1);
  }
  return solutions + ((columns == magicQ) as int)
}

#[test]
fn test_nQueens() {
  let mut solutions = Vec::new();
  for num in range(0, 9) {
    solutions.push(nQueens(num));
  }
  assert!(solutions == vec!(1, 1, 0, 0, 2, 10, 4, 40, 92));
}

#[bench]
fn bench_nQueens(b: &mut test::BenchHarness) {
  b.iter(|| { test::black_box(nQueens(12)); });
}