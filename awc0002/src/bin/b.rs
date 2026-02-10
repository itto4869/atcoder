use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
        b: [Usize1; m],
    }
    let mut cnt = 0;
    let mut a_sum = 0;
    for &bi in &b {
        if a[bi] < k {
            cnt += 1;
            a_sum += a[bi];
        }
    }

    println!("{} {}", cnt, a_sum);
}
