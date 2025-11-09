use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        mut abc: [u64; 3],
    }
    let mut ans = u64::MAX;
    let l = 9999;
    abc.sort_unstable();
    abc.reverse();
    for x in 0..=l {
        for y in 0..=(l - x) {
            if n >= (abc[0] * x + abc[1] * y) && (n - (abc[0] * x + abc[1] * y)) % abc[2] == 0 {
                ans = ans.min(x + y + ((n - (abc[0] * x + abc[1] * y)) / abc[2]));
            }
        }
    }
    println!("{}", ans);
}
