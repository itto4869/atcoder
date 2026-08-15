use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut n_v = Vec::new();
    let mut p_v = Vec::new();
    for _ in 0..n {
        input! {
            a: i64,
        }
        if a.is_negative() {
            n_v.push(a);
        } else {
            p_v.push(a);
        }
    }

    n_v.sort_unstable();
    n_v.reverse();
    p_v.sort_unstable();
    let mut n_i = 0;
    let mut p_i = 0;
    let mut prev = 0i64;
    let mut ans = 0;
    while n_i < n_v.len() && p_i < p_v.len() {
        let n_a = n_v[n_i];
        let p_a = p_v[p_i];

        if prev.abs_diff(n_a) < prev.abs_diff(p_a) {
            ans += prev.abs_diff(n_a);
            prev = n_a;
            n_i += 1;
        } else if prev.abs_diff(n_a) > prev.abs_diff(p_a) {
            ans += prev.abs_diff(p_a);
            prev = p_a;
            p_i += 1;
        } else {
            ans += prev.abs_diff(n_a);
            prev = n_a;
            n_i += 1;
        }
    }

    if n_i < n_v.len() {
        for i in n_i..n_v.len() {
            let n_a = n_v[i];
            ans += prev.abs_diff(n_a);
            prev = n_a;
        }
    }

    if p_i < p_v.len() {
        for i in p_i..p_v.len() {
            let p_a = p_v[i];
            ans += prev.abs_diff(p_a);
            prev = p_a;
        }
    }

    println!("{}", ans);
}
