use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: Usize1,
        s: [String; n],
    }
    let mut res: Vec<String> = Vec::with_capacity(n.pow(k as u32));
    let mut buf: Vec<usize> = vec![0; k];
    dfs(0, n, &s, &mut res, &mut buf);
    res.sort();
    let ans = &res[x];
    println!("{}", ans);
}

fn dfs(r: usize, n: usize, s: &Vec<String>, res: &mut Vec<String>, buf: &mut Vec<usize>) {
    if r == buf.len() {
        let mut u = String::with_capacity(50);
        for &mut i in buf {
            let temp = &s[i];
            u.push_str(temp);
        }
        res.push(u);
    } else {
        for i in 0..n {
            buf[r] = i;
            dfs(r + 1, n, s, res, buf);
        }
    }
}