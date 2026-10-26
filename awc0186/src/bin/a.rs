use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let ans = a.into_iter().filter(|x| x % 2 == 1).count();
    println!("{}", ans);
}
