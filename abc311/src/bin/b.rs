use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Bytes; n],
    }
    let mut ans = 0;
    let mut cnt = 0;
    for j in 0..d {
        let mut ok = true;
        for i in 0..n {
            if s[i][j] == b'x' {
                ok = false;
                break;
            }
        }

        if ok {
            cnt += 1;
            ans = ans.max(cnt);
        } else {
            cnt = 0;
        }
    }

    println!("{}", ans);
}
