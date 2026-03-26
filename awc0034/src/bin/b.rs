use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
    }
    let mut curr = 0;
    let mut ans = 1;
    while curr < (n - 1) {
        curr = p[curr];
        ans += 1;
    }

    println!("{}", ans);
}
