use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut prefix_sum = 0;
    a.insert(0, 0);
    a.push(0);
    for i in 0..=n {
        prefix_sum += a[i + 1].abs_diff(a[i]);
    }

    for i in 1..=n {
        let ans = prefix_sum - a[i].abs_diff(a[i - 1]) - a[i + 1].abs_diff(a[i]) + a[i + 1].abs_diff(a[i - 1]);
        println!("{}", ans);
    }
}
