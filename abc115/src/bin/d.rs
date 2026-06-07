use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u64,
    }

    let mut len = vec![0u64; n + 1];
    let mut patty = vec![0u64; n + 1];

    len[0] = 1;
    patty[0] = 1;

    for i in 1..=n {
        len[i] = len[i - 1] * 2 + 3;
        patty[i] = patty[i - 1] * 2 + 1;
    }

    let ans = solve(n, x, &len, &patty);
    println!("{}", ans);
}

fn solve(n: usize, x: u64, len: &Vec<u64>, patty: &Vec<u64>) -> u64 {
    if x == 0 {
        return 0;
    }

    if n == 0 {
        return 1;
    }

    // レベル n バーガー:
    // B + burger(n-1) + P + burger(n-1) + B

    if x == 1 {
        // 最初の B だけ
        0
    } else if x <= 1 + len[n - 1] {
        // 左側の burger(n-1) の途中
        solve(n - 1, x - 1, len, patty)
    } else if x == 1 + len[n - 1] + 1 {
        // 左 burger(n-1) 全部 + 真ん中の P
        patty[n - 1] + 1
    } else if x <= 1 + len[n - 1] + 1 + len[n - 1] {
        // 左 burger(n-1) 全部 + 真ん中 P + 右 burger(n-1) の途中
        patty[n - 1] + 1 + solve(n - 1, x - 1 - len[n - 1] - 1, len, patty)
    } else {
        // 全部食べた
        patty[n]
    }
}