use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a: Vec<usize> = (1..=n).collect();
    let mut shift = 0;
    for _ in 0..q {
        input! {
            c: u64,
        }

        if c == 1 {
            input! {
                p: Usize1,
                x: usize,
            }
            let idx = (p + shift) % n;
            a[idx] = x;
        } else if c == 2 {
            input! {
                p: Usize1,
            }
            let idx = (p + shift) % n;
            println!("{}", a[idx]);
        } else {
            input! {
                k: usize,
            }
            shift = (shift + k) % n;
        }
    }
}
