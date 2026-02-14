use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    }
    let mut a = 1;
    while a * a * a < n {
        let a3 = a * a * a;
        if (n - a3) % (3 * a) == 0 {
            let b = (n - a3) / (3 * a);
            let d = a * a + 4 * b;
            let sq = d.isqrt();
            if sq * sq != d {
                a += 1;
                continue;
            }

            if sq <= a {
                a += 1;
                continue;
            }
            if (sq - a) % 2 != 0 {
                a += 1;
                continue;
            }

            let y = (sq - a) / 2;
            let x = a + y;
            println!("{} {}", x, y);
            return;

        } else {
            a += 1;
        }
    }

    println!("-1");
}
