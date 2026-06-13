use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort_unstable();
    let mut min_y = 1usize << 60;
    let mut ans = 0usize;
    for (_, y) in xy {
        if y < min_y {
            ans += 1;
        }

        min_y = y.min(min_y);
    }

    println!("{}", ans);
}
