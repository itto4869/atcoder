use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c1: String,
        c2: String,
        s: [Bytes; h],
    }
    let mut ans = String::new();
    for i in 0..h {
        for _ in 0..k {
            for j in 0..w {
                if s[i][j] == b'#' {
                    ans.push_str(&c1.repeat(k));
                } else {
                    ans.push_str(&c2.repeat(k));
                }
            }
            ans.push('\n');
        }
    }

    print!("{}", ans);
}
