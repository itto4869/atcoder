use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![0; 101];
    for _ in 0..n {
        input! {
            a: usize,
        }
        v[a] += 1;
    }

    let mut ans = 0;
    for i in 1..=100 {
        if (v[i] % 2) == 1 {
            ans += i;
        }
    }

    println!("{}", ans);
}
