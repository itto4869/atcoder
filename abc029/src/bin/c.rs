use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut s = String::new();
    dfs(n, &mut s);
}

fn dfs(res: usize, s: &mut String) {
    if res == 0 {
        println!("{}", s);
    } else {
        for c in ['a', 'b', 'c'] {
            s.push(c);
            dfs(res - 1, s);
            s.pop();
        }
    }
}