use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    dfs(0, 0, n, 0, &mut ans);
    println!("{}", ans);
}

fn dfs(x: usize, digit: u32, n: usize, ok: usize, ans: &mut usize) {
    let a = x + 3 * 10usize.pow(digit);
    let b = x + 5 * 10usize.pow(digit);
    let c = x + 7 * 10usize.pow(digit);

    if a <= n {
        let ok = ok | 1;
        if (ok & 1 == 0) || (ok & (1 << 1) == 0) || (ok & (1 << 2) == 0) {
            dfs(a, digit + 1, n, ok, ans);
        } else {
            *ans = (*ans) + 1;
            dfs(a, digit + 1, n, ok, ans);
        }
    } 

    if b <= n {
        let ok = ok | (1 << 1);
        if (ok & 1 == 0) || (ok & (1 << 1) == 0) || (ok & (1 << 2) == 0) {
            dfs(b, digit + 1, n, ok, ans);
        } else {
            *ans = (*ans) + 1;
            dfs(b, digit + 1, n, ok, ans);
        }
    }

    if c <= n {
        let ok = ok | (1 << 2);
        if (ok & 1 == 0) || (ok & (1 << 1) == 0) || (ok & (1 << 2) == 0) {
            dfs(c, digit + 1, n, ok, ans);
        } else {
            *ans = (*ans) + 1;
            dfs(c, digit + 1, n, ok, ans);
        }
    }
}