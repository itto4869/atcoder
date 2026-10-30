use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut l = 0;
    let mut r = 0;
    let mut map = HashMap::new();
    let mut ans = 0;
    while r < n {
        let ar = a[r];
        let v = map.get(&ar).unwrap_or(&0);
        map.insert(ar, v + 1);

        while map.len() > k {
            let al = a[l];
            let &v = map.get(&al).unwrap();
            if v == 1 {
                map.remove(&al);
            } else {
                map.insert(al, v - 1);
            }

            l += 1;
        }
        ans = ans.max(r - l + 1);
        r += 1;
    }

    println!("{}", ans);
}
