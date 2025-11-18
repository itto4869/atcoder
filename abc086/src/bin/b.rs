use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: String,
        b: String,
    }
    a.push_str(&b);
    let n: u64 = a.parse().unwrap();
    let mut i = 1;
    while i * i <= n {
        if i * i == n {
            println!("Yes");
            return;
        } else {
            i += 1;
        }
    }
    println!("No");
}
