use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        y: usize,
    }

    for a in 0..=n {
        for b in 0..=(n - a) {
            let price = 10000 * a + 5000 * b + 1000 * (n - a - b);
            if price == y {
                println!("{} {} {}", a, b, n - a - b);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
