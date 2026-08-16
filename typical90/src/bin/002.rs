use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut s = Vec::new();
    let mut v = Vec::new();
    dfs(&mut s, 0, 0, n, &mut v);
    v.sort_unstable();
    println!("{}", v.iter().format("\n"));
}

fn dfs(s: &mut Vec<char>, l_cnt: usize, r_cnt: usize, n: usize, v: &mut Vec<String>) {
    if s.len() == n {
        if l_cnt != r_cnt {
            return;
        } else {
            let ss: String = s.iter().collect();
            v.push(ss);
        }
    } else {
        if l_cnt == r_cnt {
            s.push('(');
            dfs(s, l_cnt + 1, r_cnt, n, v);
            s.pop();
        } else {
            s.push('(');
            dfs(s, l_cnt + 1, r_cnt, n, v);

            s.pop();
            s.push(')');
            dfs(s, l_cnt, r_cnt + 1, n, v);
            s.pop();
        }
    }
}