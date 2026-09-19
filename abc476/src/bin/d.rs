use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: u128,
        x: u128,
        y: u128,
        mut a: [u128; n],
        mut b: [u128; m],
    }
    let mut imos_a = vec![0; n];
    a.sort_unstable();
    imos_a[0] = a[0];
    for i in 1..n {
        imos_a[i] = imos_a[i - 1] + a[i];
    }

    b.sort_unstable();

    let mut money = x + k * y;
    let mut ans = imos_a.partition_point(|&x| x <= money);
    let mut res = y;
    for i in 0..m {
        let use_k = (b[i] + k - 1) / k;
        if use_k > res {
            break;
        }

        let r = use_k * k - b[i];
        money = money + r - use_k * k;

        res = res - use_k;
        ans = ans.max(imos_a.partition_point(|&x| x <= money) + i + 1);
    }

    println!("{}", ans);
}
