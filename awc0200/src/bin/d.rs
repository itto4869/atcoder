use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let sum = a.iter().sum::<usize>();
    let mut ans = 0;
    for i in 0..n {
        let d = sum - a[i];
        if (n - 1) * a[i] >= k * d {
            ans += 1;
        }
    }

    println!("{}", ans);
}
