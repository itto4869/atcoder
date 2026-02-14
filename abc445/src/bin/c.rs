use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        if ans[i] != 0 {
            continue;
        }
        let mut idx = i;
        let mut idxes = Vec::new();
        loop {
            idxes.push(idx);
            let next = a[idx];
            if idx == next {
                for j in idxes {
                    ans[j] = next + 1;
                }
                break;
            } else {
                idx = next;
            }
        }
    }

    println!("{}", ans.iter().format(" "));
}
