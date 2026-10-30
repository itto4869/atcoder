use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for _ in 0..m {
        input! {
            b: Usize1,
        }
        ans += a[b];
    }

    println!("{}", ans);
}
