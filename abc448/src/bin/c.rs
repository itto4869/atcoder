use itertools::sorted;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }
    let sorted_a: Vec<u64> = sorted(a.clone()).collect();
    for _ in 0..q {
        input! {
            k: usize,
        }
        let mut sub_a = Vec::new();
        for _ in 0..k {
            input! {
                b: Usize1,
            }
            sub_a.push(a[b]);
        }
        sub_a.sort_unstable();
        let mut ans = sorted_a[k];
        for i in 0..k {
            if sorted_a[i] < sub_a[i] {
                ans = sorted_a[i];
                break;
            }
        }

        println!("{}", ans);
    }
}
