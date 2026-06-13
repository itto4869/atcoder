use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: usize,
            b: usize,
            x: i64,
            y: i64,
        }
        let x_move = x.abs() as usize;
        let y_move = y.abs() as usize;
        let d = x_move.min(y_move);

        let mut ans = 0;
        ans += 2 * d * a.min(b);

        let res = x_move.max(y_move) - d;

        let mut ans1 = ans;
        let mut ans2 = ans;
        if a > b {
            if x_move < y_move {
                ans1 += b * 2 * res;
                if res % 2 != 0 {
                    ans1 -= b;
                }
            } else {
                ans1 += b * 2 * res;
                if res % 2 != 0 {
                    ans1 += b;
                }
            }
        } else {
            if x_move < y_move {
                ans1 += a * 2 * res;
                if res % 2 != 0 {
                    ans1 += a;
                }
            } else {
                ans1 += a * 2 * res;
                if res % 2 != 0 {
                    ans1 -= a;
                }
            }
        }
        ans2 += a * (res / 2) + b * (res / 2);
        if res % 2 == 1 {
            if x_move > y_move {
                ans2 += a;
            } else {
                ans2 += b;
            }
        }

        let ans = ans1.min(ans2);
        println!("{}", ans);
    }
}
