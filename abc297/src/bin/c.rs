use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut ans = String::new();
    for _ in 0..h {
        input! {
            s: Chars,
        }
        let mut f = 0;
        for c in s {
            if c == 'T' {
                if f == 0 {
                    f = 1;
                } else {
                    ans.push('P');
                    ans.push('C');
                    f = 0;
                }
            } else {
                if f == 1 {
                    ans.push('T');
                }
                ans.push('.');
                f = 0;
            }
        }

        if f == 1 {
            ans.push('T');
        }
        ans.push('\n');
    }

    print!("{}", ans);
}
