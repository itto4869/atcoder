use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    for _ in 0..n {
        input! {
            m: usize,
            s: [usize; m],
        }
        let res = s.iter().filter(|&&x| x >= k).count();
        println!("{}", res);
    }
}
