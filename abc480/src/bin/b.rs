use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut v = vec![0; n];
    for _ in 0..m {
        input! {
            t: Chars,
        }
        let mut u = Vec::new();
        let mut ok = true;
        for i in 0..n {
            if s[i].len() != t.len() {
                continue;
            }

            let mut cnt = 0;
            for j in 0..t.len() {
                if s[i][j] != t[j] {
                    cnt += 1;
                }
            }

            if cnt == 0 {
                v[i] += 1;
                ok = false;
            } else if cnt == 1 {
                u.push(i);
            }
        }

        if u.len() == 1 && ok {
            v[u[0]] += 1;
        }
    }

    println!("{}", v.iter().format("\n"));
}
