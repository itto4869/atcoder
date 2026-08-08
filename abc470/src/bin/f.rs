use std::collections::VecDeque;

use ac_library::{Mod998244353, StaticModInt};
use proconio::{fastout, input, marker::{Chars, Usize1}};

type Mod998 = StaticModInt<Mod998244353>;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut ans = Mod998::new(1);
    for v in 0..n {
        if seen[v] {
            continue;
        }

        seen[v] = true;

        let mut cnts = vec![0usize; 26];
        let mut queue = VecDeque::new();
        queue.push_back(v);
        while let Some(v) = queue.pop_front() {
            let next = &graph[v];
            for &u in next {
                if seen[u] {
                    continue;
                }

                seen[u] = true;
                cnts[s[u] as usize - 'a' as usize] += 1;
                queue.push_back(u);
            }
        }

        let mut l = 0usize;
        let mut dupl_cnt = 0usize;
        let mut dupl_len = 0;

        for cnt in cnts {
            if cnt == 0 {
                continue;
            }

            l += 1;
            if cnt >= 2 {
                dupl_cnt += 1;
                dupl_len += cnt;
            }
        }

        let l = l.saturating_sub(dupl_len).saturating_sub(1);
        let mut ans_res = Mod998::new(0);
        for swap_n in 1..=(l.saturating_sub(1)) {
            let mut res = Mod998::new(1);
            res *= Mod998::new(l);
            res *= (l - 1).pow(swap_n as u32 - 1);
            for li in 1..=l {
                res *= (l - 1).pow(li as u32);
            }

            ans_res += res;
        }

        for i in 1..=dupl_cnt {
            ans_res *= Mod998::new(i);
        }

        ans *= ans_res;
    }

    println!("{}", ans);
}