use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        csf: [(u64, u64, u64); n - 1],
    }
    
    for i in 0..n {
        let mut res = 0;
        for j in i..(n - 1) {
            let min_f_mul = (((res + csf[j].2 - 1) / csf[j].2) * csf[j].2).max(csf[j].1);
            res = min_f_mul + csf[j].0;
        }
        println!("{}", res);
    }
}