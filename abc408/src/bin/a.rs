use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: u64,
    }
    let mut ok = true;
    let mut prev = 0;
    for _ in 0..n {
        input! {
            t: u64,
        }
        let diff = t - prev;
        prev = t;
        if s < diff {
            ok = false;
        } else {
            continue;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
