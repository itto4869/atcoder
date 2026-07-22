use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut min_a = usize::MAX;
    let mut offset = 0;
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            s: String,
            a: usize,
        }
        if a < min_a {
            min_a = a;
            offset = i;
        }

        v.push(s);
    }

    for i in 0..n {
        println!("{}", v[(i + offset) % n]);
    }
}
