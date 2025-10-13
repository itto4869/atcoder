use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut cost = 0;
    for i in 0..n {
        let mut num = a[i];
        let mut exp_num = 0;
        while num % 2 == 0 && num > 0 {
            exp_num += 1;
            num /= 2;
        }
        cost += exp_num.min(2);
    }
    if cost >= 2 * (a.len() / 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
