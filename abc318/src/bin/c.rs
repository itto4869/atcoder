use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        p: u64,
        mut f: [u64; n],
    }
    f.sort_unstable();
    f.reverse();

    let mut ans = 0;
    for i in (0..n).step_by(d) {
        let mut sum_f = 0;
        for j in 0..d {
            if i + j < n {
                sum_f += f[i + j];
            } else {
                break;
            }
        }

        if sum_f > p {
            ans += p;
        } else {
            ans += sum_f;
        }
    }

    println!("{}", ans);
}
