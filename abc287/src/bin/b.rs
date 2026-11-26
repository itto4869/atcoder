use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    let mut ans = 0;
    for i in 0..n {
        let mut ok = false;
        for j in 0..m {
            if s[i][3..] == t[j] {
                ok = true;
                break;
            }
        }

        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
