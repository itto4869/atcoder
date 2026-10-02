use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut idx = 1;
    let mut max_v = 0;
    for i in 1..=n {
        input! {
            h: usize,
            s: usize,
            _: usize,
        }

        let v = h + s;
        if v > max_v {
            max_v = v;
            idx = i;
        }
    }

    println!("{}", idx);
}
