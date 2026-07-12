use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: u128,
    }
    for _ in 0..t {
        input! {
            n: u128,
        }
        let mut ok = 0u128;
        let mut ng = 10u128.pow(19);
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            let k = if mid == 0 {
                0
            } else {
                2 * ((2 * 1 + (mid - 1) * 2) * mid)
            };
            if k > n {
                ng = mid;
            } else {
                ok = mid;
            }
        }

        let k = if ok == 0 {
            0
        } else {
            2 * ((2 * 1 + (ok - 1) * 2) * ok)
        };

        let pk = if ok == 0 || ok == 1 {
            0
        } else {
            ((2 * 2 + (ok - 2) * 2) * (ok - 1)) / 2
        };
        let res = n - k;
        let mut ans = k;
        ans += 4 * pk;

        if res == 0 {
            println!("{}", ans);
        } else {
            ans = ans + res - 1;
            ans = ans + res - (res / (ok * 2 + 1));

            println!("{}", ans);
        }
    }
}