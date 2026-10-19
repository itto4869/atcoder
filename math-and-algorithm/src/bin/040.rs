use std::mem::swap;

use cp_library::data_structure::prefix_sum::prefix_sum;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        m: usize,
        mut bi: Usize1,
    }
    let psum = prefix_sum(&a);
    let mut ans = 0;
    for _ in 0..(m - 1) {
        input! {
            mut bj: Usize1,
        }
        let next = bj;
        if bj < bi {
            swap(&mut bi, &mut bj);
        };
        
        ans += psum[bj] - psum[bi];
        
        bi = next
    }

    println!("{}", ans);
}
