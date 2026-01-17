use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut status = String::from("logout");

    let mut ans = 0;
    for _ in 0..n {
        input! {
            s: String,
        }
        if s == "login" || s == "logout" {
            status = s;
        } else if s == "private" {
            if status == "logout" {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
