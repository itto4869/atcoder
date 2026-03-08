use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..n {
        input! {
            t: usize,
            x: u64,
        }
        if t == 0 {
            a.push(x);
        } else if t == 1 {
            b.push(x);
        } else {
            c.push(x as usize);
        }
    }

    a.sort_unstable();
    a.reverse();

    b.sort_unstable();
    b.reverse();

    c.sort_unstable();
    c.reverse();
    
    let mut a_sum = vec![0; a.len() + 1];
    let mut b_sum = vec![0; b.len() + 1];
    let mut c_sum = vec![0; c.len() + 1];

    for i in 0..a.len() {
        a_sum[i + 1] = a_sum[i] + a[i];
    }

    for i in 0..b.len() {
        b_sum[i + 1] = b_sum[i] + b[i];
    }

    for i in 0..c.len() {
        c_sum[i + 1] = c_sum[i] + c[i];
    }

    let mut ans = 0;
    
    for k in 0..=b.len().min(m) {
        let r = c_sum.partition_point(|&x| x < k);

        if r == c_sum.len() {
            continue;
        }

        if k + r > m {
            continue;
        }

        let rem = m - k - r;
        let take_a = rem.min(a.len());

        let score = b_sum[k] + a_sum[take_a];
        ans = ans.max(score);
    }

    println!("{}", ans);
}
