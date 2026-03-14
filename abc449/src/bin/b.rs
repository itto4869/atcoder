use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut curr_h = h;
    let mut curr_w = w;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                r: usize,
            }
            let res = r * curr_w;
            curr_h -= r;
            println!("{}", res);
        } else {
            input! {
                c: usize,
            }
            let res = c * curr_h;
            curr_w -= c;
            println!("{}", res);
        }
    }
}
