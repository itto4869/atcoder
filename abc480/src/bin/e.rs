use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let mut x = 3;
    let mut v = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    while x * (x + 1) * (x + 2) <= n {
        v.push(v.last().unwrap() * x);
    }
    while x * (x + 1) * (x + 2) <= n {
        let a = v[x];
        let idx = v[..=x].partition_point(|&p | a - p < n);
        let d = x - idx + 1;
        let k = (d - 1) / 2;
        
    }
}
