use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ans_map = HashMap::new();
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            a: usize,
            d: usize,
            b: usize,
        }
        *ans_map.entry(a).or_insert(0) += 1;
        map.entry(d).or_insert(Vec::new()).push((a, b));
    }

    for d in 1..=m {
        if let Some(bs) = map.get(&d) {
            for (ai, bi) in bs {
                if ans_map[ai] == 1 {
                    ans_map.remove(ai);
                } else {
                    ans_map.insert(*ai, ans_map[ai] - 1);
                }

                *ans_map.entry(*bi).or_insert(0) += 1;
            }
        }

        println!("{}", ans_map.len());
    }
}
