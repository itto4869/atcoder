use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut lucas = vec![0u128; n + 1];
    lucas[0] = 2;
    lucas[1] = 1;
    for i in 2..=n {
        lucas[i] = lucas[i - 1] + lucas[i - 2];
    }
    println!("{}", lucas[n]);
}
