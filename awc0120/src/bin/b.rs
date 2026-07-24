use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: Usize1,
        a: [i64; n],
    }
    let mut imos = vec![0i64; n];
    imos[0] = a[0];
    for i in 1..n {
        imos[i] = imos[i - 1] + a[i];
    }

    let mut l = d.saturating_sub(k - 1);
    let mut ans = i64::MIN;
    while l <= d && l + k - 1 < n {
        if l == 0 {
            ans = ans.max(imos[l + k - 1]);
        } else {
            ans = ans.max(imos[l + k - 1] - imos[l - 1]);
        }

        l += 1;
    }

    println!("{}", ans);
}
