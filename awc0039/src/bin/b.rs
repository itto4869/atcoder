use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        t: u64,
    }
    let mut cnts = vec![0; n];
    let mut points = vec![0; n];
    for _ in 0..m {
        input! {
            c: Usize1,
            s: u64,
        }
        points[c] += s;
        cnts[c] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        if points[i] < t * cnts[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
