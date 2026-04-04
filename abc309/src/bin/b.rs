use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        grid: [Chars; n],
    }
    let mut ans = String::new();
    for i in 0..n {
        for j in 0..n {
            if i == 0 && j == 0 {
                ans.push(grid[1][0]);
            } else if i == 0 {
                ans.push(grid[0][j - 1]);
            } else if j == (n - 1) {
                ans.push(grid[i - 1][j]);
            } else if i == (n - 1) {
                ans.push(grid[n - 1][j + 1]);
            } else if j == 0 {
                ans.push(grid[i + 1][0]);
            } else {
                ans.push(grid[i][j]);
            }
        }
        ans.push('\n');
    }

    print!("{}", ans);
}
