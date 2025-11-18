use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut prev = (0, 0, 0);
    for _ in 0..n {
        input! {
            t: u64,
            x: u64,
            y: u64,
        }
        let dist = x.abs_diff(prev.1) + y.abs_diff(prev.2);
        let elapsed_time = t - prev.0;

        if dist > elapsed_time {
            println!("No");
            return;
        } else if (elapsed_time - dist) % 2 == 0 {
            prev = (t, x, y);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
