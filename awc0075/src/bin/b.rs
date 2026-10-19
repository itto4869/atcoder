use cp_library::data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let psum = prefix_sum(&a);
    let mut ans = usize::MAX;
    for i in 0..=(n - k) {
        ans = ans.min(psum[i + k] - psum[i]);
    }

    println!("{}", ans);
}
