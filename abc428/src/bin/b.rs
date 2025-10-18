use std::collections::HashMap;
use itertools::Itertools;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Bytes,
    }
    let mut count = HashMap::new();
    for i in 0..=(n - k) {
        let str = s[i..i+k].iter().map(|&s| s as char).collect::<String>();
        *count.entry(str).or_insert(0) += 1;
    }
    let mut count: Vec<(&String, &u64)> = count.iter().collect();
    count.sort_by(|a, b| a.1.cmp(&b.1));
    count.reverse();
    let &x = count[0].1;
    let mut x_vec = Vec::new();
    for &(c, &num) in &count {
        if num == x {
            x_vec.push(c);
        }
    }
    x_vec.sort();
    println!("{}", x);
    println!("{}", x_vec.iter().format(" "));
}
