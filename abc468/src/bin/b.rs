use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        m: usize,
        d: usize,
        s: Chars,
    }
    let mut v = vec![false; m];
    for i in 0..m {
        let c = s[i];
        if c == 'G' {
            for j in i.saturating_sub(d)..=(i + d).min(m - 1) {
                v[j] = true;
            }
        }
    }

    let ans = v.iter().filter(|&&x| !x).count();
    println!("{}", ans);
}
