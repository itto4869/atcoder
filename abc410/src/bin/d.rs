use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            w: usize,
        }
        graph[a].push((b, w));
    }

    let mut seen = vec![vec![false; 1024]; n];
    dfs(0, 0, &graph, &mut seen);

    let mut ans = usize::MAX;
    for x in 0..1024 {
        if seen[n - 1][x] {
            ans = x;
            break;
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(v: usize, x: usize, graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<Vec<bool>>) {
    let next = &graph[v];
    for &(u, w) in next {
        let nx = x ^ w;
        if !seen[u][nx] {
            seen[u][nx] = true;
            dfs(u, nx, graph, seen);
        }
    }
}