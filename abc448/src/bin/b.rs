use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [u64; m]
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: Usize1,
            b: u64,
        }
        ans += c[a].min(b);
        c[a] = c[a].saturating_sub(b);
    }

    println!("{}", ans);
}
