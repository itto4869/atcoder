use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort_unstable();
    for i in 1..n {
        if a[i] != a[i - 1] + 1 {
            println!("{}", a[i - 1] + 1);
            return;
        }
    }
}
