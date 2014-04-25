extern crate test;

use std::vec_ng::Vec;
use std::iter::AdditiveIterator;

fn main() {
    println!("{0}", semiParallelNQueens(15));
}

fn nQueens(n: i16) -> i16 {
    return nQueensHelper((1 << n) -1, 0, 0, 0);
}

fn semiParallelNQueens(n: i16) -> i16 {
    let magicQ = (1 << n) - 1;
    let columns = 0;
    let leftDiags = 0;
    let rightDiags = 0;
    let mut receivers = Vec::new(); 
    let mut validSpots = !(leftDiags | columns | rightDiags) & magicQ;
    while validSpots != 0 {
        let spot = -validSpots & validSpots;
        validSpots = validSpots ^ spot;
        let (tx, rx) = channel();
        receivers.push(rx);
        spawn(proc() {
            tx.send(nQueensHelper(
                magicQ,
                (leftDiags | spot) << 1,
                (columns | spot),
                (rightDiags | spot) >> 1));
        });
    }
    let mut results = Vec::new();
    for receiver in receivers.iter() {
        println!("Received!");
        results.push(receiver.recv());
    }
    return results.iter().map(|&x| x).sum() + ((columns == magicQ) as i16)
}

fn nQueensHelper(magicQ: i16, leftDiags: i16, columns: i16, rightDiags: i16) -> i16 {
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
    return solutions + ((columns == magicQ) as i16)
}

#[test]
fn test_nQueens() {
    let mut solutions = Vec::new();
    for num in range(0, 9i16) {
        solutions.push(nQueens(num));
    }
    assert!(solutions == vec!(1, 1, 0, 0, 2, 10, 4, 40, 92i16));
}

#[bench]
fn bench_nQueens(b: &mut test::BenchHarness) {
    b.iter(|| { test::black_box(nQueens(12)); });
}

#[bench]
fn bench_semiParallelNQueens(b: &mut test::BenchHarness) {
    b.iter(|| { test::black_box(semiParallelNQueens(12)); });
}
