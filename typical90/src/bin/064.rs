use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut diff = vec![0; n - 1];
    for i in 0..(n - 1) {
        diff[i] = a[i] - a[i + 1];
    }
    let mut res: i64 = diff.iter().map(|&x| x.abs()).sum();
    
    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            v: i64,
        }
        let mut d = 0;
        if l > 0 {
            d += (diff[l - 1] - v).abs() - diff[l - 1].abs();
            diff[l - 1] -= v;
        }

        if r < n - 1 {
            d += (diff[r] + v).abs() - diff[r].abs();
            diff[r] += v;
        }

        res += d;
        println!("{}", res);
    }
}
