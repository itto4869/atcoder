use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut cnt = vec![0; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        if a < b {
            cnt[b] += 1;
        } else if a > b {
            cnt[a] += 1;
        }
    }

    let ans = cnt.iter().filter(|&&x| x == 1).count();
    println!("{}", ans);
}
