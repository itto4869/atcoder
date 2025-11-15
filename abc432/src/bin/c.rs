use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        mut a: [u64; n],
    }
    a.sort();
    let max_a0 = (a[0] * y) as i128;
    let diff = y - x;
    let mut ans = a[0];
    for &ai in a.iter().skip(1) {
        let min_ai = (ai * x) as i128;
        let diff_ai = max_a0 - min_ai;
        if diff_ai < 0 {
            println!("-1");
            return;
        } else if diff_ai as u64 % diff != 0 {
            println!("-1");
            return;
        } else {
            ans += diff_ai as u64 / diff;
        }
    }
    println!("{}", ans);
}
