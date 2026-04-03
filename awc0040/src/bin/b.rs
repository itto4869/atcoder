use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        m: usize,
        s: u64,
        mut b: [u64; m],
        n: usize,
    }
    let q =  s / (m as u64);
    let r = s as usize % m;
    for i in 0..m {
        if i < r {
            b[i] += q + 1;
        } else {
            b[i] += q;
        }
    }

    let mut b_sum = vec![0; m + 1];
    for i in 0..m {
        b_sum[i + 1] = b_sum[i] + b[i];
    }

    for _ in 0..n {
        input! {
            l: Usize1,
            r: usize,
        }
        let ans = b_sum[r] - b_sum[l];
        println!("{}", ans);
    }
}
