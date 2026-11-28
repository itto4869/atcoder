use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut ans = 0;
    for i in 0..k {
        let mut idx = i;
        let mut f = false;
        while idx < n {
            if s[idx] == '.' {
                f = false;
            } else {
                if !f {
                    ans += 1;
                    f = true;
                }
            }

            idx += k;
        }
    }

    println!("{}", ans);
}
