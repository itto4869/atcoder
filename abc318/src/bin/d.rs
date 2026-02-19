use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut d = vec![Vec::new(); n - 1];
    for i in 0..(n - 1) {
        for _ in 0..(n - i - 1) {
            input! {
                s: u64,
            }
            d[i].push(s);
        }
    }

    let mut ans = 0;
    if n % 2 == 0 {
        dfs(0, 1, &d, 0, n, &mut ans, 1, None);
    } else {
        for i in 1..n {
            dfs(0, 1, &d, 0, n, &mut ans, 1, Some(i));
        }
        dfs(1, 2, &d, 0, n, &mut ans, 1, None);
    }
    println!("{}", ans);
}

fn dfs(idx: usize, mask: usize, d: &Vec<Vec<u64>>, res: u64, n: usize, ans: &mut u64, cnt: usize, skip: Option<usize>) {
    for i in (idx + 1)..n {
        if mask & (1 << i) != 0 {
            continue;
        }

        if cnt == (n / 2) {
            *ans = (*ans).max(res + d[idx][i - idx - 1]);
            continue;
        }

        for j in (idx + 1)..(n - 1) {
            if mask & (1 << j) == 0 && j != i {
                if let Some(k) = skip {
                    if j == k {
                        continue;
                    }
                }
                dfs(j, mask | (1 << i) | (1 << j), d, res + d[idx][i - idx - 1], n, ans, cnt + 1, skip);
                break;
            }
        }
    }
}