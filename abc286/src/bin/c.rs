use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }
    let mut ans = usize::MAX;
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..(n / 2) {
            if s[(i + j) % n] != s[(n - j - 1 + i) % n] {
                cnt += 1;
            }
        }

        ans = ans.min(a * i + b * cnt);
    }

    println!("{}", ans);
}
