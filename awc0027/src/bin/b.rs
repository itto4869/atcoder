use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut curr: u64,
    }
    let mut ans = -1;
    for i in 2..=n {
        input! {
            a: u64,
        }
        if curr < a {
            curr = a;
            ans = i as isize;
        }
    }

    println!("{}", ans);
}
