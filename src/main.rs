#[cfg(test)] extern crate test;
#[cfg(test)] use test::Bencher;

#[cfg(not(test))]
fn main() {
    println!("{}", parallel_n_queens(16));
}

/*                                                      _
   _ __     __ _ _   _  ___  ___ _ __  ___    ___  ___ | |_   _____ _ __
  | '_ \   / _` | | | |/ _ \/ _ \ '_ \/ __|  / __|/ _ \| \ \ / / _ \ '__/
  | | | | | (_| | |_| |  __/  __/ | | \__ \  \__ \ (_) | |\ V /  __/ |
  |_| |_|  \__, |\__,_|\___|\___|_| |_|___/  |___/\___/|_| \_/ \___|_|
              | |
              |_|
*/

// Solves n-queens using a depth-first, backtracking
// solution. Returns the number of solutions for a
// given n.
fn n_queens(n: i32) -> uint {
    // Pass off to our helper function.
    return n_queens_helper((1 << n as uint) -1, 0, 0, 0);
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
fn n_queens_helper(all_ones: i32, left_diags: i32, columns: i32, right_diags: i32) -> uint {
    // all_ones is a special value that simply has all 1s in the first
    // n positions and 0s elsewhere. We can use it to clear out
    // areas that we don't care about.

    // Our solution count.
    // This will be updated by the recursive calls to our helper.
    let mut solutions = 0;


    // We get valid_spots with some bit trickery. Effectively, each
    // of the parameters can be ORed together to create
    // an integer with all the conflicts together, which we then
    // invert and limit by ANDing with all_ones, our special value from
    // earlier.
    let mut valid_spots = !(left_diags | columns | right_diags) & all_ones;


    // Since valid_spots contains 1s in all of the locations that
    // are conflict-free, we know we have gone through all of
    // those locations when valid_spots is all 0s, i.e. when it is
    // 0.
    while valid_spots != 0 {
        // This is just bit trickery. For reasons involving the weird
        // behavior of two's complement integers, this creates an integer
        // which is all 0s except for a single 1 in the position of the
        // LSB of valid_spots.
        let spot = -valid_spots & valid_spots;

        // We then XOR that integer with the valid_spots to flip it to 0
        // in valid_spots.
        valid_spots = valid_spots ^ spot;


        // Make a recursive call. This is where we infer the conflicts
        // for the next row.
        solutions += n_queens_helper(
            all_ones,
            // We add a conflict in the current spot and then shift left,
            // which has the desired effect of moving all of the conflicts
            // that are created by left diagonals to the left one square.
            (left_diags | spot) << 1,

            // For columns we simply mark this column as filled by ORing
            // in the currentSpot.
            (columns | spot),

            // This is the same as the leftDiag shift, except we shift
            // right because these conflicts are caused by right
            // diagonals.
            (right_diags | spot) >> 1);
    }

    // If columns is all blocked (i.e. if it is all ones) then we
    // have arrived at a solution because we have placed n queens.
    return solutions + ((columns == all_ones) as uint)
}

// This is the same as the regular n_queens except it creates
// n threads in which to to do the work.
//
// This is much slower for smaller numbers (under 16~17) but overcomes
// the sequential algorithm after that.
fn parallel_n_queens(n: i32) -> uint {
    let all_ones = (1 << n as uint) - 1;
    let columns = 0;
    let left_diags = 0;
    let right_diags = 0;

    let mut receivers = Vec::new();
    let mut valid_spots = !(left_diags | columns | right_diags) & all_ones;
    while valid_spots != 0 {
        let spot = -valid_spots & valid_spots;
        valid_spots = valid_spots ^ spot;
        let (tx, rx) = channel();
        receivers.push(rx);
        spawn(proc() {
            tx.send(n_queens_helper(
                all_ones,
                (left_diags | spot) << 1,
                (columns | spot),
                (right_diags | spot) >> 1));
        });
    }

    let mut results = 0u;
    for receiver in receivers.iter() {
        results += receiver.recv();
    }
    return results + ((columns == all_ones) as uint)
}

// Tests

#[test]
fn test_n_queens() {
    let real = vec![1, 1, 0, 0, 2, 10, 4, 40, 92u];
    for num in range(0, 9i32) {
        assert!(n_queens(num) == real[num as uint]);
    }
}

#[test]
fn test_parallel_n_queens() {
    let real = vec![1, 1, 0, 0, 2, 10, 4, 40, 92u];
    for num in range(0, 9i32) {
        assert!(parallel_n_queens(num) == real[num as uint]);
    }
}

#[bench]
fn bench_n_queens(b: &mut Bencher) {
    b.iter(|| { test::black_box(n_queens(12)); });
}

#[bench]
fn bench_parallel_n_queens(b: &mut Bencher) {
    b.iter(|| { test::black_box(parallel_n_queens(12)); });
}
