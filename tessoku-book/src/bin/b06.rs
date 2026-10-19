use cp_library::data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let psum = prefix_sum(&a);
    for _ in 0..q {
        input! {
            l: Usize1,
            r: usize,
        }
        let cnt = 2 * (psum[r] - psum[l]);
        let len = r - l;
        if cnt > len {
            println!("win");
        } else if cnt < len {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
