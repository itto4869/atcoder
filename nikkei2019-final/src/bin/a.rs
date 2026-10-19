use cp_library::data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let psum = prefix_sum(&a);
    for k in 1..=n {
        let mut res = 0;
        for i in 0..=(n - k) {
            res = res.max(psum[i + k] - psum[i]);
        }
        println!("{}", res);
    }
}
