use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ps = Vec::with_capacity(n);
    for i in 1..=n {
        input! {
            p: usize,
            s: usize,
        }
        ps.push((p, s, i));
    }

    ps.sort_by(|a, b| {
        (a.0 * b.1).cmp(&(b.0 * a.1))
    });

    let ans = ps[0].2;

    println!("{}", ans);
}
