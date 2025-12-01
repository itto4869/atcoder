use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut i = s.len();
    let mut j = t.len();
    let mut ans = Vec::new();

    while i > 0 && j > 0 {
        if s[i - 1] == t[j - 1] {
            ans.push(s[i - 1] as char);
            i -= 1;
            j -= 1;
        } else if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
