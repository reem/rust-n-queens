extern crate test;

use std::vec_ng::Vec;
use std::iter::AdditiveIterator;

fn main() {
    println!("{0}", semiParallelNQueens(15));
}

/*           _
   ___  ___ | |_   _____ _ __
  / __|/ _ \| \ \ / / _ \ '__/
  \__ \ (_) | |\ V /  __/ |
  |___/\___/|_| \_/ \___|_|

*/

// Solves n-queens using a depth-first, backtracking
// solution. Returns the number of solutions for a
// given n.
fn nQueens(n: i16) -> i16 {
    // Pass off to our helper function.
    return nQueensHelper((1 << n) -1, 0, 0, 0);
}

// The meat of the algorithm is in here, a recursive helper function
// that actually computes the answer using a depth-first, backtracking
// algorithm.
//
// The 30,000 foot overview is as follows:
//
// This function takes only 3 important parameters: three integers
// which represent the spots on the current row that are blocked
// by previous queens.
//
// The "secret sauce" here is that we can avoid passing around the board
// or even the locations of the previous queens and instead we use this
// information to infer the conflicts for the next row.
//
// Once we know the conflicts in our current row we can simply recurse
// over all of the open spots and profit.
//
// This implementation is optimized for speed and memory by using
// integers and bit shifting instead of arrays for storing the conflicts.
fn nQueensHelper(allOnes: i16, leftDiags: i16, columns: i16, rightDiags: i16) -> i16 {
    // allOnes is a special value that simply has all 1s in the first
    // n positions and 0s elsewhere. We can use it to clear out
    // areas that we don't care about.

    // Our solution count.
    // This will be updated by the recursive calls to our helper.
    let mut solutions = 0;


    // We get validSpots with some bit trickery. Effectively, each
    // of the parameters can be ORed together to create
    // an integer with all the conflicts together, which we then
    // invert and limit by ANDing with allOnes, our special value from
    // earlier.
    let mut validSpots = !(leftDiags | columns | rightDiags) & allOnes;


    // Since validSpots contains 1s in all of the locations that
    // are conflict-free, we know we have gone through all of
    // those locations when validSpots is all 0s, i.e. when it is
    // 0.
    while validSpots != 0 {
        // This is just bit trickery. For reasons involving the weird
        // behavior of two's complement integers, this creates an integer
        // which is all 0s except for a single 1 in the position of the
        // LSB of validSpots.
        let spot = -validSpots & validSpots;

        // We then XOR that integer with the validSpots to flip it to 0
        // in validSpots.
        validSpots = validSpots ^ spot;


        // Make a recursive call. This is where we infer the conflicts
        // for the next row.
        solutions += nQueensHelper(
            allOnes,
            // We add a conflict in the current spot and then shift left,
            // which has the desired effect of moving all of the conflicts
            // that are created by left diagonals to the left one square.
            (leftDiags | spot) << 1,

            // For columns we simply mark this column as filled by ORing
            // in the currentSpot.
            (columns | spot),

            // This is the same as the leftDiag shift, except we shift
            // right because these conflicts are caused by right
            // diagonals.
            (rightDiags | spot) >> 1);
    }

    // If columns is all blocked (i.e. if it is all ones) then we
    // have arrived at a solution because we have placed n queens.
    return solutions + ((columns == allOnes) as i16)
}

// This is the same as the regular nQueens except it creates
// n threads in which to to do the work.
//
// This is 10x slower on my machine, though your mileage may vary.
fn semiParallelNQueens(n: i16) -> i16 {
    let allOnes = (1 << n) - 1;
    let columns = 0;
    let leftDiags = 0;
    let rightDiags = 0;

    let mut receivers = Vec::new();
    let mut validSpots = !(leftDiags | columns | rightDiags) & allOnes;
    while validSpots != 0 {
        let spot = -validSpots & validSpots;
        validSpots = validSpots ^ spot;
        let (tx, rx) = channel();
        receivers.push(rx);
        spawn(proc() {
            tx.send(nQueensHelper(
                allOnes,
                (leftDiags | spot) << 1,
                (columns | spot),
                (rightDiags | spot) >> 1));
        });
    }

    let mut results = Vec::new();
    for receiver in receivers.iter() {
        results.push(receiver.recv());
    }
    return results.iter().map(|&x| x).sum() + ((columns == allOnes) as i16)
}

// Tests

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
