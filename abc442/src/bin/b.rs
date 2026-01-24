use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut v = 0u64;
    let mut s = false;
    for _ in 0..q {
        input! {
            a: u64,
        }
        if a == 1 {
            v += 1;
        } else if a == 2 {
            v = v.saturating_sub(1);
        } else {
            if s {
                s = false;
            } else {
                s = true;
            }
        }

        if v >= 3 && s {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
