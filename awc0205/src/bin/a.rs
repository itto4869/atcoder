use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = String::new();
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        if (a + b) % 2 == 0 {
            ans.push('o');
        } else {
            ans.push('x');
        }
    }

    println!("{}", ans);
}
