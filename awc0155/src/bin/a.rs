use proconio::{fastout, input, marker::Chars};
use cp_library;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 1;
    let mut max_cnt = 0;
    for i in 1..=n {
        input! {
            s: Chars,
        }

        if s.len() < 8 {
            continue;
        }
        let mut cnt = 0;
        let tanabata = ['t', 'a', 'n', 'a', 'b', 'a', 't', 'a'];
        for i in 0..(s.len() - 7) {
            if s[i..(i + 8)] == tanabata {
                cnt += 1;
            }
        }

        if max_cnt < cnt {
            ans = i;
            max_cnt = cnt;
        }
    }

    println!("{}", ans);
}
