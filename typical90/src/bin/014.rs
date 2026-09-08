use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    a.sort_unstable();
    b.sort_unstable();

    let mut ans = 0;
    for i in 0..n {
        ans += a[i].abs_diff(b[i]);
    }

    println!("{}", ans);
}
