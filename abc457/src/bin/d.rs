use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u128,
        a: [u128; n],
    }
    let mut ok = *(a.iter().min().unwrap());
    let mut ng = 10u128.pow(36);
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(&a, mid, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn f(a: &Vec<u128>, x: u128, k: u128) -> bool {
    let mut cnt = 0;
    let mut ok = true;
    for i in 0..a.len() {
        let ai = a[i];
        let d = x.saturating_sub(ai);
        cnt += (d + i as u128) / (i as u128 + 1);
        if cnt > k {
            ok = false;
            break;
        }
    }

    ok
}