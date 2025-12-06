use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut ans = 0;
    for l in 0..n {
        for r in l..n {
            let mut sum_a = 0;
            let mut ok = true;
            for j in l..=r {
                sum_a += a[j];
            }

            for i in l..=r {
                if sum_a % a[i] == 0 {
                    ok = false;
                    break;
                }
            }

            if ok {
                ans += 1;
            } else {
                continue;
            }
        }
    }

    println!("{}", ans);
}
