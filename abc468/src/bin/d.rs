use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut v = vec![vec![0u32; s.len()]; s.len()];
    let mut ans = 0;
    for d in 0..s.len() {
        for i in 0..(s.len() - d) {
            if d == 0 {
                v[i][i] = 0;
                ans += 1;
            } else {
                let c1 = s[i];
                let c2 = s[i + d];
                if c1 == c2 {
                    let f = v[i + 1][i + d - 1];
                    if f == 0 || f == 1 {
                        ans += 1;
                        v[i][i + d] = f;
                    } else {
                        v[i][i + d] = 2;
                    }
                } else {
                    let f = v[i + 1][i + d - 1];
                    if f == 0 {
                        ans += 1;
                        v[i][i + d] = 1;
                    } else {
                        v[i][i + d] = 2;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
