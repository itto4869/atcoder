use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [u64; 3],
        mut k: u64,
    }
    abc.sort();
    while k > 0 {
        abc[2] *= 2;
        k -= 1;
    }

    let ans: u64 = abc.iter().sum();
    println!("{}", ans);
}
