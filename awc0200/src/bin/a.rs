use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = String::new();
    let mut c = 'R';
    for i in 1..=n {
        ans.push(c);
        if i % k == 0 {
            if c == 'R' {
                c = 'B';
            } else {
                c = 'R';
            }
        }
    }

    println!("{}", ans);
}
