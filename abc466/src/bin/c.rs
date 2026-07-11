use proconio::{fastout, input};

fn main() {
    input! {
        n: usize,
    }
    let mut l = 1;
    let mut r = 2;
    let mut ans = 0;
    for _ in 0..(2 * n) {
        println!("? {} {}", l, r);

        input! {
            s: String,
        }

        if s == "Yes" {
            ans += r - l;
            r += 1;
        } else {
            l += 1;
            if l == r {
                r += 1;
            }
        }

        if r > n {
            break;
        }
    }

    println!("! {}", ans);
}
