use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut map = vec![n; n];
    let mut back_map = vec![n; n];
    let mut v = vec![true; n];
    for _ in 0..q {
        input! {
            c: Usize1,
            p: Usize1,
        }
        v[c] = false;
        if back_map[c] == n {
            map[p] = c;
            back_map[c] = p;
        } else {
            map[p] = c;
            map[back_map[c]] = n;
            back_map[c] = p;
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        if v[i] {
            let mut u = i;
            let mut cnt = 1;
            while map[u] != n {
                u = map[u];
                cnt += 1;
            }

            ans[i] = cnt;
        } else {
            ans[i] = 0;
        }
    }

    println!("{}", ans.iter().format(" "));
}
