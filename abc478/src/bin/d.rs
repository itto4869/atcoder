use cp_library::data_structure::intervalset::IntervalSet;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut lrx: [(Usize1, Usize1, usize); q],
    }
    
    let mut v = vec![Vec::new(); q + 1];
    for (l, r, x) in lrx {
        v[x].push((l as i64, r as i64));
    }

    let mut set = IntervalSet::new();
    let mut imos = vec![0; n + 1];

    for lr in v {
        set.clear();
        for (l, r) in lr {
            set.insert(l, r + 1);
        }

        for (l, r) in set.iter() {
            imos[l as usize] += 1;
            imos[r as usize] -= 1;
        }
    }

    for i in 1..n {
        imos[i] += imos[i - 1];
    }
    
    imos.pop();
    
    println!("{}", imos.iter().format(" "));
}
