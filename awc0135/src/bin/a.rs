use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize,
        a: [usize; n],
    }
    let &min_a = a.iter().min().unwrap();
    let mut ans = 0;
    for ai in a {
        ans += ai - min_a;
    }

    println!("{}", ans);
}
