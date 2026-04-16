use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut y: u64,
    }
    loop {
        if y % 4 == 2 {
            println!("{}", y);
            break;
        } else {
            y += 1;
        }
    }
}
