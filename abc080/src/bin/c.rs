use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        f: [[u8; 10]; n],
        p: [[i64; 11]; n],
    }
    let mut ans = i64::MIN;
    for bit in 1..(1 << 10) {
        let mut profit = 0;
        for i in 0..n {
            let mut num = 0;
            for j in 0..10 {
                if f[i][j] == 1 && bit & (1 << j) != 0 {
                    num += 1;
                }
            }
            profit += p[i][num];
        }
        ans = ans.max(profit);
    }
    println!("{}", ans);
}
