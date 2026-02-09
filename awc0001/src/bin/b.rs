use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        p: [i64; n],
    }
    let mut idx = -1;
    let mut max_p = -1;
    for i in 0..n {
        let pi = p[i];
        if l <= pi && pi <= r && pi > max_p {
            max_p = pi;
            idx = (i + 1) as isize;
        }
    }

    println!("{}", idx);
}
