use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut xs: Vec<i64> = xy.iter().map(|&(x, _)| x).collect();
    let mut ys: Vec<i64> = xy.iter().map(|&(_, y)| y).collect();

    xs.sort();
    ys.sort();

    let (c_x, c_y) = if n % 2 == 0 {
        (((xs[n / 2 - 1] + xs[n / 2]) / 2), (ys[n / 2 - 1] + ys[n / 2]) / 2)
    } else {
        (xs[n / 2], ys[n / 2])
    };

    let mut ans = 0;
    for &(x, y) in &xy {
        ans += x.abs_diff(c_x) + y.abs_diff(c_y);
    }

    println!("{}", ans);
}
