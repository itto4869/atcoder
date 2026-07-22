use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _w: usize,
        _h: usize,
        n: usize,
        pq: [(usize, usize); n],
        an: usize,
        a: [usize; an],
        bn: usize,
        b: [usize; bn],
    }

    // (縦方向のピース番号, 横方向のピース番号) -> イチゴ数
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();

    for &(p, q) in &pq {
        // p より左にある切断線の本数
        let x = a.partition_point(|&cut| cut < p);

        // q より下にある切断線の本数
        let y = b.partition_point(|&cut| cut < q);

        *counts.entry((x, y)).or_insert(0) += 1;
    }

    let max_ans = *counts.values().max().unwrap();

    let total_pieces = (an as u64 + 1) * (bn as u64 + 1);

    let min_ans = if (counts.len() as u64) < total_pieces {
        // HashMapに存在しない、イチゴ0個のピースがある
        0
    } else {
        // 全ピースに少なくとも1個載っている
        *counts.values().min().unwrap()
    };

    println!("{} {}", min_ans, max_ans);
}