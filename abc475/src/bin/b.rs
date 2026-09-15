use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut cnts = vec![0; 3];
    for _ in 0..n {
        input! {
            a: usize,
        }
        if a % 1000 == 0 {
            continue;
        }
        let r = a / 1000;
        let r = 1000 * (r + 1) - a;
        cnts[2] += r / 100;
        let r = r % 100;
        cnts[1] += r / 10;
        let r = r % 10;
        cnts[0] += r;
    }
    
    println!("{}", cnts.iter().format(" "));
}
