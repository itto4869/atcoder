use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        grid: [Chars; 3],
    }
    println!("{}{}{}", grid[0][0], grid[1][1], grid[2][2]);
}
