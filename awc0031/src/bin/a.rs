use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut u = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            v: u64,
            c: usize,
            t: [Usize1; c],
        }
        u.push((v, t));
    }
    
    u.sort_by(|a, b| {
        if a.0.cmp(&b.0) == std::cmp::Ordering::Equal {
            std::cmp::Ordering::Less
        } else {
            a.0.cmp(&b.0)
        }
    });

    u.reverse();
    let mut w = vec![0; m];
    for i in 0..k {
        for &ti in &u[i].1 {
            w[ti] += 1;
        }
    }

    let ans = w.into_iter().filter(|&x| x == k).count();
    println!("{}", ans);
}
