use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut x: usize,
            mut y: usize,
            k: usize,
        }
        if x == y {
            println!("{}", 0);
            continue;
        }
        let mut map = HashMap::new();
        let mut cnt = 1usize;
        map.insert(x, 0);
        while x > 0 {
            x = x / k;
            map.insert(x, cnt);
            cnt += 1;
        }

        cnt = 0;
        if map.contains_key(&y) {
            let ans = map[&y];
            println!("{}", ans);
            continue;
        }
        while y > 0 {
            y = y / k;
            cnt += 1;
            if map.contains_key(&y) {
                let x_cnt = map[&y];
                let ans = cnt + x_cnt;
                println!("{}", ans);
                break;
            }
        }
    }
}
