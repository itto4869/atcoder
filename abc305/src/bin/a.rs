use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let mut min = n;
    for x in (5..=100).step_by(5) {
        if n.abs_diff(x) < min {
            ans = x;
            min = n.abs_diff(x);
        }
    }

    println!("{}", ans);
}
