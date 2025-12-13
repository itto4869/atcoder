use std::collections::HashSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut set = HashSet::new();
    let mut ans = 0;
    for _ in 0..m {
        input! {
            r: Usize1,
            c: Usize1,
        }
        let points = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];
        let mut ok = true;
        for p in points {
            if set.contains(&p) {
                ok = false;
                break;
            }
        }

        if ok {
            for p in points {
                set.insert(p);
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
