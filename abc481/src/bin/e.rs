use std::cmp::Ordering::{Equal, Greater, Less};
use cp_algorithm::polar_angle_sort;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            xy: [(i64, i64); n],
        }
        let p = polar_angle_sort(&xy).repeat(2);
        let mut r = 1;
        let mut ans = 0;
        for l in 0..n {
            r = r.max(l + 1);
            let (x, y) = p[l];

            while r < l + n {
                let (u, v) = p[r];
                if x * v <= y * u {
                    break;
                }
                r += 1;
            }
            ans = ans.max(r - l);
        }

        println!("{}", ans);
    }

}
