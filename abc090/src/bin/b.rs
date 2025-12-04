use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    for n in a..=b {
        if is_palindromic(n) {
            ans += 1;
        } else {
            continue;
        }
    }

    println!("{}", ans);
}

fn is_palindromic(mut n: usize) -> bool {
    let mut cs = Vec::new();
    while n > 0 {
        cs.push(n % 10);
        n /= 10;
    }
    let vec1 = cs.clone();
    cs.reverse();
    if vec1 == cs {
        true
    } else {
        false
    }
}
