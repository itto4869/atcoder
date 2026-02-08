use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut imos = vec![0; 2 * 100000 + 1];
    for idx in a {
        imos[0] += 1;
        imos[idx] -= 1;
    }

    for i in 0..(imos.len() - 1) {
        imos[i + 1] += imos[i];
    }

    for i in 0..(imos.len() - 1) {
        if imos[i] >= 10 {
            let k = imos[i] / 10;
            imos[i + 1] += k;
            imos[i] -= k * 10;
        }
    }

    let mut ans = Vec::new();
    imos.reverse();
    let idx = imos.iter().position(|&x| x > 0).unwrap();
    for i in idx..imos.len() {
        ans.push(imos[i]);
    }

    println!("{}", ans.iter().format(""));
}
