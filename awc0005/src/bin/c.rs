use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut moved = a.clone();
    for i in 0..(n - 1) {
        if moved[i].abs_diff(moved[i + 1]) > k {
            if moved[i] > moved[i + 1] {
                moved[i + 1] += moved[i] - moved[i + 1] - k;
            } else {
                moved[i] += moved[i + 1] - moved[i] - k;
            }
        }
    }

    for i in (1..n).rev() {
        if moved[i].abs_diff(moved[i - 1]) > k {
            if moved[i] > moved[i - 1] {
                moved[i - 1] += moved[i] - moved[i - 1] - k;
            } else {
                moved[i] += moved[i - 1] - moved[i] - k;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans += moved[i] - a[i];
    }
    
    println!("{}", ans);
}
