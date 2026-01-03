use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut i_map = HashMap::new();
    let mut j_map = HashMap::new();
    let mut k_map = HashMap::new();

    for i in 0..n {
        let ai = a[i];
        if ai % 7 == 0 {
            i_map.entry(ai / 7).or_insert(Vec::new());
            if let Some(val) = i_map.get_mut(&(ai / 7)) { val.push(i); };
        } 
        if ai % 5 == 0 {
            j_map.entry(ai / 5).or_insert(Vec::new());
            if let Some(val) = j_map.get_mut(&(ai / 5)) { val.push(i); };
        } 
        if ai % 3 == 0 {
            k_map.entry(ai / 3).or_insert(Vec::new());
            if let Some(val) = k_map.get_mut(&(ai / 3)) { val.push(i); };
        }
    }

    let mut ans = 0;

    for (r, j_idxes) in j_map {
        if i_map.get(&r).is_none() {
            continue;
        }

        if k_map.get(&r).is_none() {
            continue;
        }

        let i_idxes = i_map.get(&r).unwrap();
        let k_idxes = k_map.get(&r).unwrap();

        for j_idx in j_idxes {
            let i_num_max = i_idxes.partition_point(|&x| x < j_idx);
            let k_num_max = k_idxes.partition_point(|&x| x < j_idx);
            ans += i_num_max * k_num_max;

            let i_num_min = i_idxes.len() - i_num_max;
            let k_num_min = k_idxes.len() - k_num_max;
            ans += i_num_min * k_num_min;
        }
    }

    println!("{}", ans);

}
