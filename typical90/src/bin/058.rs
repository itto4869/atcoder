use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut x = n;
    let mut map = HashMap::new();
    let mut nums = Vec::new();
    let mut ans = x;
    for i in 0..k {
        nums.push(x);
        map.insert(x, i);
        let mut res = x;
        while x > 0 {
            res += x % 10;
            x /= 10;
        }
        ans = res % 100000;
        if map.contains_key(&ans) {
            let i = i + 1;
            let j = map.get(&ans).unwrap();
            let d = i - j;
            let idx = (k - i) % d + j;
            ans = nums[idx];
            break;
        } else {
            x = ans;
        }
    }

    println!("{}", ans);
}
