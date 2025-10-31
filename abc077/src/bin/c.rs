use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
    }
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();
    let mut upper_num = HashMap::new();
    let mut num = 0;
    for &bi in b.iter().rev() {
        let idx = c.partition_point(|&x| x <= bi);
        num += n - idx;
        upper_num.insert(bi, num);
    }
    let mut ans = 0;
    for &ai in &a {
        let idx = b.partition_point(|&x| x <= ai);
        if idx == n { continue; }
        ans += upper_num.get(&b[idx]).unwrap();

    }
    println!("{}", ans);
}
