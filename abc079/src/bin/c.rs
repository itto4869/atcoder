use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        abcd: Bytes,
    }
    let nums: Vec<u8> = abcd.iter().map(|&b| b - b'0').collect();
    for mask in 0..(1 << 3) {
        let mut n = nums[0] as i64;
        let mut ans = nums[0].to_string();
        for i in 1..4 {
            if mask & (1 << i - 1) == 0 {
                ans.push_str(&format!("+{}", nums[i]));
                n += nums[i] as i64;
            } else {
                ans.push_str(&format!("-{}", nums[i]));
                n -= nums[i] as i64;
            }
        }

        if n == 7 {
            println!("{}=7", ans);
            return;
        }
    }
}
