use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map: HashMap<char, u64> = HashMap::new();
    for _ in 0..n {
        input! {
            s: Bytes,
        }
        
        match s[0] {
            b'M' => *map.entry('M').or_insert(0) += 1,
            b'A' => *map.entry('A').or_insert(0) += 1,
            b'R' => *map.entry('R').or_insert(0) += 1,
            b'C' => *map.entry('C').or_insert(0) += 1,
            b'H' => *map.entry('H').or_insert(0) += 1,
            _ => {}
        }
    }

    let size = map.len();
    if size < 3 {
        println!("0");
        return;
    }

    let mut ans = 0;
    for comb in map.into_iter().combinations(3) {
        ans += comb[0].1 * comb[1].1 * comb[2].1;
    }

    println!("{}", ans);
}
