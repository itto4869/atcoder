use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }
    let (c0, r0) = (c[0], r[0]);
    let mut ans_0 = 0;
    let mut ans_0_v = 0;
    let mut ans_1 = 1;
    let mut ans_1_v = r0;
    for (idx, (&c, r)) in c.iter().zip(r).enumerate() {
        if c == t {
            if ans_0_v < r {
                ans_0 = idx + 1;
                ans_0_v = r;
            }
        } else if c == c0 {
            if ans_1_v < r {
                ans_1 = idx + 1;
                ans_1_v = r;
            }
        }
    }

    if ans_0_v > 0 {
        println!("{}", ans_0);
    } else {
        println!("{}", ans_1);
    }
}
