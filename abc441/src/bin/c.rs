use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: u64,
        mut a: [u64; n],
    }
    a.sort();
    let mut a_sum = 0;
    for i in 0..k {
        a_sum += a[i];
    }

    if a_sum < x {
        println!("-1");
        return;
    }

    let mut drink = vec![false; n];
    if a_sum >= x && (a_sum - a[0]) < x {
        drink[0] = true;
    }
    for i in (k - 1)..n {
        drink[i] = true;
    }

    a_sum = a[k - 1];
    for i in (0..k - 1).rev() {
        if a_sum < x {
            drink[i] = true;
        } else {
            break;
        }
        a_sum += a[i];
    }

    let ans = drink.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
