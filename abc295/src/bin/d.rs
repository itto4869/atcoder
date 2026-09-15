use std::collections::HashMap;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut v = vec![vec![0; 10]; s.len()];
    
    v[0][(s[0] - b'0') as usize] = 1;
    for i in 1..s.len() {
        let n = (s[i] - b'0') as usize;
        for j in 0..10 {
            if j == n {
                v[i][j] = v[i - 1][j] + 1;
            } else {
                v[i][j] = v[i - 1][j];
            }
        }
    }

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    let mut ans = 0usize;
    map1.insert(0, 1);
    for i in 0..s.len() {
        let mut bit = 0usize;
        for j in 0..10 {
            if (v[i][j] % 2) == 1 {
                bit = (1 << j) | bit;
            }
        }

        if (i % 2) == 0 {
            ans += map2.get(&bit).unwrap_or(&0);
            *map2.entry(bit).or_insert(0) += 1;
        } else {
            ans += map1.get(&bit).unwrap_or(&0);
            *map1.entry(bit).or_insert(0) += 1;
        }
    }

    println!("{}", ans);
}
