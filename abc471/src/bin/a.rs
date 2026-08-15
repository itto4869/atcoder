use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if ((a + b) == 9) || ((a.saturating_sub(b) == 9)) || ((a * b) == 9) || ((a % b == 0) && (a / b == 9)) {
        println!("Nine");
    } else {
        println!("Nein");
    }
}
