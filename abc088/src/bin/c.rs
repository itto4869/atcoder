use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        grid: [[u64; 3]; 3],
    }
    let diff11 = grid[0][0].abs_diff(grid[0][1]);
    let diff12 = grid[0][1].abs_diff(grid[0][2]);
    let diff21 = grid[1][0].abs_diff(grid[1][1]);
    let diff22 = grid[1][1].abs_diff(grid[1][2]);
    let diff31 = grid[2][0].abs_diff(grid[2][1]);
    let diff32 = grid[2][1].abs_diff(grid[2][2]);

    if diff11 == diff21 && diff21 == diff31 && diff12 == diff22 && diff22 == diff32 {
        println!("Yes");
    } else {
        println!("No");
    }
}
