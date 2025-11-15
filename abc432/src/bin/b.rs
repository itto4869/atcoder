use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: u64,
    }
    let mut nums = Vec::new();
    let mut cnt_zero = 0;
    while x > 0 {
        if x % 10 == 0 {
            cnt_zero += 1;
        } else {
            nums.push(x % 10);
        }
        x /= 10;
    }
    nums.sort();
    print!("{}", nums[0]);
    for _ in 0..cnt_zero {
        print!("0");
    }
    for &n in nums.iter().skip(1) {
        print!("{}", n);
    }
}
