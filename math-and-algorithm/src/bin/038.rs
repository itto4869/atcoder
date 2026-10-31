use cp_data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let psum = prefix_sum(&a);
    for _ in 0..q {
        input! {
            l: Usize1,
            r: usize,
        }

        println!("{}", psum[r] - psum[l]);
    }
}
