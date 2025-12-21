use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort();
    let ai = a[n - 1];
    let mut aj = 0;
    for &ak in &a {
        if ak.min(ai - ak) > aj {
            aj = ak;
        }
    }
    println!("{} {}", ai, aj);
}
