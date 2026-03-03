use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let pi = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    let mut pi = pi.chars();
    let mut ans = String::from("3.");
    for _ in 0..n {
        ans.push(pi.next().unwrap());
    }

    println!("{}", ans);
}
