use cp_library::math::prime_enumeration::enumerate_primes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let primes = enumerate_primes(n.isqrt());
    let m = primes.len();
    let mut ans = 0usize;
    let mut ok = true;
    for i in 0..(m - 2) {
        let mut ook = true;
        for j in (i + 1)..(m - 1) {
            for k in (j + 1)..m {
                let (a, b, c) = (primes[i] as u128, primes[j] as u128, primes[k] as u128);
                let x = a * a * b * c * c;
                if x <= n as u128 {
                    ans += 1;
                } else {
                    if k == j + 1 {
                        ook = false;
                    }
                    break;
                }
            }

            if !ook {
                if j == i + 1 {
                    ok = false;
                }
                break;
            }
        }

        if !ok {
            break;
        }
    }

    println!("{}", ans);
}
