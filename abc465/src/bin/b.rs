use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        l: Usize1,
        r: Usize1,
        a: Usize1,
        b: Usize1,
    }
    let mut v = vec![y; 24];
    for i in l..r {
        v[i] = x;
    }

    let mut ans = 0;
    for i in a..b {
        ans += v[i];
    }

    println!("{}", ans);
}
