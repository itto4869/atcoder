use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }
    let mut v = vec![0; n];
    for ai in a {
        v[ai] += 1;
    }

    v.sort_unstable();
    v.reverse();
    let mut ans = 0;
    for i in k..n {
        ans += v[i];
    }

    println!("{}", ans);
}
