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
    let mut ans = 0;
    for r in 1..=n {
        let r_sum = psum[r];
        if r_sum < k {
            continue;
        }

        let res = r_sum - k;
        let ll = psum.partition_point(|&x| x < res);
        let lr = psum.partition_point(|&x| x <= res);
        ans += lr - ll;
    }

    println!("{}", ans);
}
