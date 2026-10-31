use cp_data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Bytes
    }
    let s: Vec<usize> = s.iter().map(|&c| (c - b'0') as usize).collect();
    let v = prefix_sum(&s);
    let mut ans = 0;
    for i in 0..=(n - m) {
        let x = v[i + m] - v[i];
        if x < k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
