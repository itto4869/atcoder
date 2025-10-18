use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            c: u64,
            d: u64,
        }
        let mut digit = 0;
        let mut n = d;
        while n > 0 {
            n /= 10;
            digit += 1;
        }

        for i in 0..digit {
            let min_value = f(c, 10u64.pow(i));
            let max_value = f(c, (10u64.pow(i + 1) - 1).min(d));
            
        }
    }
}

fn f(a: u64, b: u64) -> u128 {
    let mut digit = 0;
    let mut c = a as u128;
    let mut d = b as u128;
    while d > 0 {
        d /= 10;
        digit += 1;
    }
    let res = c * digit + d;
    res
}