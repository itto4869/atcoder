use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut a = Vec::new();
    a.push(1);
    for i in 1..=n {
        let mut aj = 0;
        for j in 0..i {
            aj += f(a[j]);
        }
        a.push(aj);
    }
    println!("{}", a[n]);
}

fn f(mut x: u64) -> u64{
    let mut res = 0;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }
    res
}