use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }
    let mut imos = vec![0; n];
    imos[0] = l[0];
    for i in 1..n {
        imos[i] = imos[i - 1] + l[i];
    }

    let mut ans = usize::MAX;
    for i in 1..n {
        ans = ans.min(imos[i - 1].abs_diff(imos[n - 1] - imos[i - 1]));
    }

    println!("{}", ans);
}
