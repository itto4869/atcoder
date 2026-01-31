use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut r: [i64; n],
        }
        let mut s = 0;
        for &ri in &r {
            s += ri;
        }

        for i in 1..n {
            r[i] = r[i].min(r[i - 1] + 1);
        }

        for i in (0..n - 1).rev() {
            r[i] = r[i].min(r[i + 1] + 1);
        }

        let mut ss = 0;
        for &ri in &r {
            ss += ri;
        }

        let ans = s - ss;
        println!("{}", ans);
    }
}