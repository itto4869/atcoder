use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut ans = 0;
    if n == 1 {
        println!("{}", ans);
    } else {
        dfs(n - 1, n - 1, 1 + x, x * y, x, y, &mut ans);
        println!("{}", ans);
    }
}

fn dfs(nr: usize, nb: usize, r: usize, b: usize, x: usize, y: usize, ans: &mut usize) {
    if nr == 1 && nb == 1 {
        *ans = (*ans).max(b);
    } else {
        dfs(nr - 1, nb - 1, r + (r * x) + b, (r * x + b) * y, x, y, ans);
    }
}