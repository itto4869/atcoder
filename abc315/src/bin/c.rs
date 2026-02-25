use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v1 = vec![0; n];
    let mut v2 = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            f: Usize1,
            s: u64,
        }
        v1[f] = v1[f].max(s);
        v2.push(s);
    }

    v1.sort_unstable();
    v1.reverse();

    v2.sort_unstable();
    v2.reverse();

    let ans = (v1[0] + v1[1]).max(v2[0] + v2[1] / 2);
    println!("{}", ans);
}
