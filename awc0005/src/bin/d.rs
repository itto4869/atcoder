use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut left = 0;
    let mut right = a.iter().sum::<u64>() + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if can_divide(mid, k, &a) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}

fn can_divide(n: u64, k: u64, a: &Vec<u64>) -> bool {
    let mut cnt = 0;
    let mut a_sum = 0;
    let mut ok = false;
    for &ai in a {
        a_sum += ai;
        if a_sum >= n {
            cnt += 1;
            a_sum = 0;
        }

        if cnt == k {
            ok = true;
            break;
        }
    }

    ok
}