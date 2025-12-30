use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![Vec::new(); 5];
    for i in 2..=55555 {
        if is_prime(i) {
            v[(i % 5) as usize].push(i);
        }
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        ans.push(v[1][i]);
    }

    println!("{}", ans.iter().format(" "));
}

fn is_prime(x: u64) -> bool {
    let mut d = 2;
    let mut ok = true;
    while d * d <= x {
        if x % d == 0 {
            ok = false;
            break;
        }

        d += 1;
    }
    ok
}