use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(Usize1, Usize1, usize, usize); q],
    }

    let mut v = Vec::new();
    let mut ans = 0;
    dfs(&mut v, n, m, &abcd, &mut ans);
    println!("{}", ans);
}

fn dfs(v: &mut Vec<usize>, n: usize, m: usize, abcd: &Vec<(usize, usize, usize, usize)>, ans: &mut usize) {
    if v.len() == n {
        let mut sum = 0;
        for &(a, b, c, d) in abcd {
            if v[b] - v[a] == c {
                sum += d;
            }
        }
        *ans = (*ans).max(sum);
    } else if v.len() == 0 {
        for a in 1..=m {
            v.push(a);
            dfs(v, n, m, abcd, ans);
            v.pop();
        }
    } else {
        for a in v[v.len() - 1]..=m {
            v.push(a);
            dfs(v, n, m, abcd, ans);
            v.pop();
        }
    }
}