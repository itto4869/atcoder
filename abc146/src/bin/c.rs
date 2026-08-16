use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }
    if x < (a + b) {
        println!("0");
        return;
    }
    let mut ok = 1;
    let mut ng = x / a + 1;
    while (ng - ok) > 1 {
        let mid = (ng + ok) / 2;
        let y = a * mid + b * (mid.ilog10() as usize + 1);
        if x < y {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    println!("{}", ok.min(1_000_000_000));
}
