use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a: Vec<u64> = (1..=(n as u64)).into_iter().collect();
    let mut offset = 0;
    for _ in 0..q {
        input! {
            c: u64,
        }
        if c == 1 {
            input! {
                p: Usize1,
                x: u64,
            }
            a[(p + offset) % n] = x;
        } else if c == 2 {
            input! {
                p: Usize1,
            }
            println!("{}", a[(p + offset) % n]);
        } else {
            input! {
                k: usize,
            }
            offset += k;
        }
    }
}
