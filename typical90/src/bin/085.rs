use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    let mut ans = 0usize;
    let mut a = 1usize;
    while a * a <= k {
        if k % a == 0 {
            let mut b = a;
            while b * b <= k / a {
                if (k / a) % b == 0 {
                    ans += 1;
                }

                b += 1;
            }
        }

        a += 1;
    }

    println!("{}", ans);
}
