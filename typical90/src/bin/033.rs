use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    if (h < 2) || (w < 2) {
        println!("{}", h * w);
    } else {
        let ans = ((w + 1) / 2) * ((h + 1) / 2);
        println!("{}", ans);
    }
}
