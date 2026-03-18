use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [i64; n],
        a: [i64; n],
        b: [i64; n],
    }
    let mut ans = a.iter().sum::<i64>() - b.iter().sum::<i64>();
    let mut diff = 0;
    for i in 0..n {
        diff = diff.max(p[i] - a[i]);
    }

    ans = ans + diff;
    println!("{}", ans);
}
