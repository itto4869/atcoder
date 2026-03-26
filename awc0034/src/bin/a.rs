use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [u64; n],
    }
    let mut ans = 0;
    for _ in 0..m {
        input! {
            t: Usize1
        }

        if c[t] > 0 {
            c[t] -= 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
