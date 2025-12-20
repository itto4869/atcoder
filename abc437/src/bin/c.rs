use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
        }
        let mut sum_p = 0;
        let mut weights = Vec::with_capacity(n);
        for _ in 0..n {
            input! {
                w: u64,
                p: u64,
            }
            sum_p += p;
            weights.push(w + p);
        }

        weights.sort();
        
        let mut ans = 0;
        let mut i = 0;
        let mut sum_w = 0;
        while i < n {
            sum_w += weights[i];
            if sum_p >= sum_w {
                ans += 1;
            } else {
                break;
            }
            i += 1;
        }
        println!("{}", ans);
    }
}
