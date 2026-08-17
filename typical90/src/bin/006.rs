use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut c = vec![vec![n + 1; 26]; n + 1];
    for i in (1..=n).rev() {
        let si = s[i - 1];
        for j in 0..26 {
            if (si as usize - 'a' as usize) == j {
                c[i - 1][j] = i;
            } else {
                c[i - 1][j] = c[i][j];
            }
        }
    }

    let mut ans = String::new();
    let mut idx = 0;
    let mut len = 0;
    while len < k {
        for j in 0..26 {
            if c[idx][j] <= (n - k + len + 1) {
                ans.push(char::from_u32('a' as u32 + j as u32).unwrap());
                idx = c[idx][j];
                len += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
