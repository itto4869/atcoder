use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut v = vec![0; n + 1];
    for i in 2..=n {
        if v[i] != 0 {
            continue;
        }

        for j in (i..=n).step_by(i) {
            v[j] += 1usize;
        }
    }

    let ans = v.into_iter().filter(|&x| x >= k).count();
    println!("{}", ans);
}
