use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: Usize1,
        s: [String; n],
    }
    let mut v = Vec::new();
    let mut curr = Vec::new();
    dfs(&mut v, n, &mut curr, &s, k);
    v.sort_unstable();
    let ans = &v[x];
    println!("{}", ans);
}

fn dfs(v: &mut Vec<String>, n: usize, curr: &mut Vec<String>, sv: &Vec<String>, res: usize) {
    if res == 0 {
        let mut tmp = String::new();
        for s in curr {
            tmp.push_str(s);
        }
        v.push(tmp);
    } else {
        for i in 0..n {
            curr.push(sv[i].clone());
            dfs(v, n, curr, sv, res - 1);
            curr.pop();
        }
    }
}