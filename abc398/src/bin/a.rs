use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = String::new();
    ans.push_str(&"-".repeat((n + 1) / 2 - 1));
    if n % 2 == 0 {
        ans.push_str("==");
    } else {
        ans.push('=');
    }

    ans.push_str(&"-".repeat((n + 1) / 2 - 1));

    println!("{}", ans);
}
