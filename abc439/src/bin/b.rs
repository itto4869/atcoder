use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut nums = HashSet::new();
    let mut x = n;
    let mut i = 0;
    while !nums.contains(&x) || i < 1000000 {
        i += 1;
        let mut new_x = 0;
        while x > 0 {
            let temp = x % 10;
            new_x += temp * temp;
            x /= 10;
        }

        x = new_x;

        if x == 1 {
            println!("Yes");
            return;
        } else {
            nums.insert(x);
        }
    }

    println!("No");
}
