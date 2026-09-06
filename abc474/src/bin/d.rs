use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ok = false;
    for i in 0..n {
        if a[i] > b[i] {
            ok = true;
            break;
        }
    }

    let mut w = Vec::new();
    for i in 0..n {
        if a[i] > b[i] {
            w.push(10usize.pow(18));
        } else {
            w.push(1);
        }
    }

    if ok {
        println!("Yes\n{}", w.iter().format(" "));
    } else {
        println!("No");
    }
}
