use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let n32 = n as u32;
    let center = (3usize.pow(n32) / 2, 3usize.pow(n32) / 2);
    let mut ans = vec![vec!['#'; 3usize.pow(n32)]; 3usize.pow(n32)];
    dfs(n, center, &mut ans);
    println!("{}", ans.iter().map(|v| v.iter().format("")).format("\n"));
}

fn dfs(k: usize, center: (usize, usize), ans: &mut Vec<Vec<char>>) {
    if k == 0 {
        ans[center.0][center.1] = '#';
    } else {
        let exp = (k - 1) as u32;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    let offset_y = center.0 - 3usize.pow(exp) / 2;
                    let offset_x = center.1 - 3usize.pow(exp) / 2;
                    for x in 0..(3usize.pow(exp)) {
                        for y in 0..(3usize.pow(exp)) {
                            ans[offset_y + y][offset_x + x] = '.';
                        }
                    }
                } else {
                    let (r, c) = (center.0 - 3usize.pow(exp) + 3usize.pow(exp) * i, center.1 - 3usize.pow(exp) + 3usize.pow(exp) * j);
                    dfs(k - 1, (r, c), ans);
                }
            }
        }
    }
}