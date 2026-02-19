use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut lef = vec![0usize; n];
    let mut rig = vec![0usize; n];

    let j = 1;
    for i in 0..n {
        if i < j {
            lef[a[i]] += 1;
        } else if i > j {
            rig[a[i]] += 1;
        } 
    }

    let mut lr_sum = 0;
    for i in 0..n {
        lr_sum += lef[i] * rig[i];
    }

    let mut ans = lr_sum - lef[a[1]] * rig[a[1]];
    for j in 2..(n - 1) {
        lr_sum -= lef[a[j - 1]] * rig[a[j - 1]];
        lef[a[j - 1]] += 1;
        lr_sum += lef[a[j - 1]] * rig[a[j - 1]];

        lr_sum -= lef[a[j]] * rig[a[j]];
        rig[a[j]] -= 1;
        lr_sum += lef[a[j]] * rig[a[j]];

        ans += lr_sum - lef[a[j]] * rig[a[j]];
    }

    println!("{}", ans);
}