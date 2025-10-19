use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [u64; n],
    }
    let mut count = 0;
    for i in 0..(n - 1) {
        if p[i] == (i + 1) as u64 {
            p.swap(i, i + 1);
            count += 1;
        }
    }

    if p[n - 1] == n as u64 { count += 1 };

    println!("{}", count);
}
