use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }
    let mut a_sum = vec![0; n];
    a_sum[0] = a[0];
    for i in 1..n {
        a_sum[i] = a_sum[i - 1] + a[i];
    }

    let mut map: HashMap<i64, u64> = HashMap::new();
    map.insert(0, 1);
    let mut ans = 0;
    for &s in &a_sum {
        let m = s - k;
        if let Some(cnt) = map.get(&m) {
            ans += cnt;
        }

        *map.entry(s).or_insert(0) += 1;
    }

    println!("{}", ans);
}
