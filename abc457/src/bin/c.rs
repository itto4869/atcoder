use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut a = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            l: usize,
        }
        for _ in 0..l {
            input! {
                ai: usize,
            }
            a[i].push(ai);
        }
    }

    let mut curr = 1;
    for i in 0..n {
        input! {
            c: usize,
        }
        let length = a[i].len();
        if curr + (length * c) > k {
            let idx = k - curr;
            let ans = a[i][idx % length];
            println!("{}", ans);
            break;
        } else {
            curr += length * c;
        }
    }
}
