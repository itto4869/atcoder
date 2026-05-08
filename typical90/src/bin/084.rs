use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut o_cnt = vec![0u64; n];
    let mut x_cnt = vec![0u64; n];
    if s[0] == 'o' {
        o_cnt[0] = 1;
    } else {
        x_cnt[0] = 1;
    }

    for i in 1..n {
        let c = s[i];
        if c == 'o' {
            o_cnt[i] = o_cnt[i - 1] + 1;
            x_cnt[i] = x_cnt[i - 1];
        } else {
            o_cnt[i] = o_cnt[i - 1];
            x_cnt[i] = x_cnt[i - 1] + 1;
        }
    }

    let mut ans = 0;
    for i in 1..n {
        if o_cnt[i] == 0 || x_cnt[i] == 0 {
            continue;
        }
        let c = s[i];
        if c == 'o' {
            let curr_x = x_cnt[i];
            let idx = x_cnt.partition_point(|&x| x < curr_x);
            ans += idx + 1;
        } else {
            let curr_o = o_cnt[i];
            let idx = o_cnt.partition_point(|&o| o < curr_o);
            ans += idx + 1;
        }
    }

    println!("{}", ans);
}
