use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    }
    let mut cnt = 0;
    let mut ans = String::new();
    for c in s {
        if c == 'x' {
            ans.push('x');
        } else if cnt < k {
            ans.push('o');
            cnt += 1;
        } else {
            ans.push('x');
        }
    }

    println!("{}", ans);
}
