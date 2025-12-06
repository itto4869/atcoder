use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut s: [u64; n],
        }
        let start = s[0];
        let target = s[n - 1];

        let mut between = if n > 2 {
            s[1..n - 1].to_vec()
        } else {
            vec![]
        };
        between.sort();

        let mut size = start;
        let mut ans = 1;
        let mut p = true;

        loop {
            if target <= 2 * size {
                ans += 1;
                break;
            }

            let idx = between.upper_bound(&(2 * size));

            if idx == 0 {
                p = false;
                break;
            }

            let next = between[idx - 1];

            if next <= size {
                p = false;
                break;
            }

            size = next;
            ans += 1;
        }

        if p {
            println!("{}", ans);
        } else {
            println!("{}", -1);
        }
    }
}
