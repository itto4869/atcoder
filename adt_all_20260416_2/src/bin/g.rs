use petgraph::algo::min_spanning_tree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }
    let mut l_sum_v = vec![0; n];
    let mut a_sum = 0;
    let mut l_sum = 0;
    for i in 0..n {
        a_sum += a[i];
        l_sum += l;
        l_sum_v[i] = l_sum - a_sum;
    }

    a_sum = 0;
    let mut r_sum_v = vec![0; n];
    let mut r_sum = 0;
    for i in (0..n).rev() {
        a_sum += a[i];
        r_sum += r;
        r_sum_v[i] = r_sum - a_sum;
    }

    let mut l_sum_min_v = Vec::new();
    let mut l_sum_min = i64::MAX;
    let mut l_sum_min_idx = 0;
    for i in 0..n {
        if l_sum_v[i] < l_sum_min {
            l_sum_min = l_sum_v[i];
            l_sum_min_idx = i;
        }

        l_sum_min_v.push((l_sum_min, l_sum_min_idx));
    }

    let mut r_sum_min_v = Vec::new();
    let mut r_sum_min = i64::MAX;
    let mut r_sum_min_idx = n - 1;
    for i in (0..n).rev() {
        if r_sum_v[i] < r_sum_min {
            r_sum_min = r_sum_v[i];
            r_sum_min_idx = i;
        }

        r_sum_min_v.push((r_sum_min, r_sum_min_idx));
    }

    r_sum_min_v.reverse();
    let mut min_sum = i64::MAX;
    for i in 0..n {
        let (l_min, l_idx) = l_sum_min_v[i];
        let (r_min, r_idx) = r_sum_min_v[i];
        if l_idx < r_idx && (l_min + r_min) < min_sum {
            min_sum = l_min + r_min;
        }
    }

    let a_sum = a.iter().sum::<i64>();
    let ans = a_sum + min_sum.min(0);

    println!("{}", ans);
}