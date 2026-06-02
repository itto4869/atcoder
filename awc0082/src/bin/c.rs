use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        t: Chars,
    }
    let mut cnt_r = 0;
    let mut cnt_b = 0;
    let mut prev = 0;
    let mut ans = 0;
    for c in t {
        if c == 'R' {
            prev = 0;
            cnt_r += 1;
        } else {
            if prev == 0 {
                ans += 1;
                prev = 1;
            }
            cnt_b += 1;
        }
    }

    if prev == 1 {
        ans -= 1;
    }

    if cnt_r != a || cnt_b != b {
        println!("-1");
    } else { 
        println!("{}", ans);
    }
}
