use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
        }
        let mut x = 1;
        let mut ans = 0;
        let mut sum = 0;
        let mut v = Vec::new();
        while x <= n {
            if (sum % 2) == 0 {
                v.push(x);
                sum ^= x;
                ans += sum;
                x += 1;
            } else {
                if x % 2 == 0 {
                    if (x + 1) <= n {
                        v.push(x + 1);
                        v.push(x);
                        sum ^= x + 1;
                        ans += sum;
                        sum ^= x;
                        ans += sum;
                        x += 2;
                    } else {
                        v.push(x);
                        sum ^= x;
                        ans += sum;
                        x += 1;
                    }
                } else {
                    v.push(x);
                    sum ^= x;
                    ans += sum;
                    x += 1;
                }
            }
        }
        println!("{}", v.iter().format(" "));
    }
}
