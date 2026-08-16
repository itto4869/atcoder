use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    if b == 0 {
        println!("0");
        return;
    }
    for x in 1..=(b.ilog2() as usize + 1) {
        let x = 2usize.pow((x - 1) as u32);
        let r = ((b + 1) / (2 * x)) * x + (((b + 1) % (2 * x)).saturating_sub(x));
        let l = (a / (2 * x)) * x + ((a % (2 * x)).saturating_sub(x));
        ans += x * ((r - l) % 2);
    }

    println!("{}", ans);
}
