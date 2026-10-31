use cp_data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let a = a.repeat(2);
    let psum = prefix_sum(&a);
    let mut ans = i64::MIN;
    for i in 0..n {
        ans = ans.max(psum[i + m] - psum[i]);
    }

    println!("{}", ans);
}
