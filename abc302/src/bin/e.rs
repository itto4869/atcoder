use std::collections::HashSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut graph = vec![HashSet::new(); n];
    let mut ans = n;
    let mut cnt = vec![0usize; n];

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }

            if cnt[u] == 0 {
                ans -= 1;
            }

            if cnt[v] == 0 {
                ans -= 1;
            }

            cnt[u] += 1;
            cnt[v] += 1;

            graph[u].insert(v);
            graph[v].insert(u);
        } else {
            input! {
                v: Usize1
            }

            let mut d_v = Vec::new();
            for &u in &graph[v] {
                d_v.push(u);
                if cnt[u] == 1 {
                    ans += 1;
                }

                cnt[u] = cnt[u].saturating_sub(1);
            }

            for u in d_v {
                graph[u].remove(&v);
            }

            if cnt[v] > 0 {
                ans += 1;
            }
            
            cnt[v] = 0;
            graph[v].clear();
            
        }

        println!("{}", ans);
    }
}
