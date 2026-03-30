use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut l = *a.iter().max().unwrap() - 1;
    let mut r = a.iter().sum::<u64>(); 

    while r - l > 1 {
        let mid = (l + r) / 2;
        if can_split(&a, mid, k) {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{}", r);
}

fn can_split(a: &Vec<u64>, mid: u64, k: u64) -> bool {
    let mut curr = 0;
    let mut cnt = 0;
    for &ai in a {
        if curr + ai > mid {
            curr = ai;
            cnt += 1;
        } else {
            curr += ai;
        }
    }

    if cnt <= k {
        true
    } else {
        false
    }
}