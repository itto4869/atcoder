use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: u64,
            mut a: [u64; n],
            mut b: [u64; n],
        }
        a.sort_unstable();
        a.reverse();
        b.sort_unstable();
        let mut c = 0;
        let mut idx = 0;
        for &v in &a {
            while idx < n && b[idx] + v < m {
                idx += 1;
            }
            if idx >= n {
                break
            }
            c += 1;
            idx += 1;
        }
        println!("{}", a.iter().sum::<u64>() + b.iter().sum::<u64>() - m * c);
    }
}
