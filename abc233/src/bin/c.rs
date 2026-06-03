use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let mut v = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            l: usize,
        }
        for _ in 0..l {
            input! {
                a: usize,
            }
            v[i].push(a);
        }
    }

    let mut ans = 0;
    dfs(x, 0, &v, &mut ans);

    println!("{}", ans);
}

fn dfs(res: usize, idx: usize, v: &Vec<Vec<usize>>, ans: &mut usize) {
    if idx == (v.len() - 1) {
        for &a in &v[idx] {
            if res == a {
                *ans = (*ans) + 1;
            }
        }
    } else {
        for &a in &v[idx] {
            if (res % a != 0) || (res < a) {
                continue;
            }

            dfs(res / a, idx + 1, v, ans);
        }
    }
}