use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }
    a.sort_unstable();
    for _ in 0..q {
        input! {
            b: usize,
        }
        let idx = a.partition_point(|&x| x < b);
        if idx == n {
            println!("{}", a[n - 1].abs_diff(b));
        } else if idx == 0 {
            println!("{}", a[idx].abs_diff(b));
        } else {
            let res = a[idx - 1].abs_diff(b).min(a[idx].abs_diff(b));
            println!("{}", res);
        }
    }
}
