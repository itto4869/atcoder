use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xr: [(i64, i64); n],
    }
    let mut up = HashSet::new();
    let mut rest = Vec::new();
    let mut ans = 0;
    for i in 0..n {
        let (x, r) = xr[i];
        if up.contains(&(x + r)) {
            rest.push((x, r));
        } else {
            up.insert(x + r);
            ans += 1;
        }
    }

    let mut rrest = Vec::new();
    for (x, r) in rest {
        let down = x - r;
        if up.contains(&down) {
            rrest.push((x, r));
        } else {
            up.insert(down);
            ans += 1;
        }
    }

    println!("{}", ans);
}
