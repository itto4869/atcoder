use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        k: Usize1
    }
    
    let mut ans = Vec::new();
    dfs(k, 0, &mut ans);
    ans.sort_unstable();
    println!("{}", ans[k]);
}

fn dfs(k: usize, x: usize, ans: &mut Vec<usize>) {
    if x == 0 {
        for i in 1..=9 {
            ans.push(i);
            dfs(k, i, ans);
        }
    } 
    else {
        let bottom = x % 10;
        for i in 0..bottom {
            ans.push(10 * x + i);
            dfs(k, 10 * x + i, ans);
        }
    }
}