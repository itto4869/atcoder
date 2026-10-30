use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut p = vec![Vec::new(); n + 1];
    for i in 1..=n {
        p[a[i - 1]].push(i);
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += (n + 1 - i) * (i / 2);
    }

    for i in 1..=n {
        if p[i].is_empty() {
            continue;
        }
        let mut l = 0;
        let mut r = p[i].len() - 1;
        while l < r {
            if p[i][l] < (n + 1 - p[i][r]) {
                ans -= (r - l) * p[i][l];
                l += 1;
            } else {
                ans -= (r - l) * (n + 1 - p[i][r]);
                r -= 1;
            }
        }
    }

    println!("{}", ans);
}
