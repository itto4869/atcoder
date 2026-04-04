use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut ans = String::new();
    for i in 0..h {
        for j in 0..w {
            if i == 0 {
                ans.push('#');
            } else if j == 0 {
                ans.push('#');
            } else if j == (w - 1) {
                ans.push('#');
            } else if i == (h - 1) {
                ans.push('#');
            } else {
                ans.push('.');
            }
        }
        ans.push('\n');
    }

    print!("{}", ans);
}
