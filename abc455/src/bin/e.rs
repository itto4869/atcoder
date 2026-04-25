use std::collections::HashMap;
use proconio::{fastout, input, marker::Chars};

fn comb2(x: i64) -> i64 {
    x * (x - 1) / 2
}

#[fastout]
fn main() {
    input! {
        n: i64,
        s: Chars,
    }

    let mut a = 0_i64;
    let mut b = 0_i64;
    let mut c = 0_i64;

    let mut ab_map: HashMap<i64, i64> = HashMap::new();
    let mut bc_map: HashMap<i64, i64> = HashMap::new();
    let mut ca_map: HashMap<i64, i64> = HashMap::new();
    let mut abc_map: HashMap<(i64, i64), i64> = HashMap::new();

    // prefix 0 を入れる
    *ab_map.entry(0).or_insert(0) += 1;
    *bc_map.entry(0).or_insert(0) += 1;
    *ca_map.entry(0).or_insert(0) += 1;
    *abc_map.entry((0, 0)).or_insert(0) += 1;

    for &ci in &s {
        match ci {
            'A' => a += 1,
            'B' => b += 1,
            'C' => c += 1,
            _ => unreachable!(),
        }

        *ab_map.entry(a - b).or_insert(0) += 1;
        *bc_map.entry(b - c).or_insert(0) += 1;
        *ca_map.entry(c - a).or_insert(0) += 1;
        *abc_map.entry((a - b, a - c)).or_insert(0) += 1;
    }

    let total = n * (n + 1) / 2;

    let mut eq_ab = 0_i64;
    let mut eq_bc = 0_i64;
    let mut eq_ca = 0_i64;
    let mut eq_abc = 0_i64;

    for (_, v) in ab_map {
        eq_ab += comb2(v);
    }

    for (_, v) in bc_map {
        eq_bc += comb2(v);
    }

    for (_, v) in ca_map {
        eq_ca += comb2(v);
    }

    for (_, v) in abc_map {
        eq_abc += comb2(v);
    }

    let ans = total - eq_ab - eq_bc - eq_ca + 2 * eq_abc;

    println!("{}", ans);
}