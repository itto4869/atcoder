use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = vec![0; n];
    let mut ts = Vec::new();
    for i in 0..n {
        let mut q = Vec::new();
        for j in 0..k {
            q.push((i + j) % n + 1);
        }
        println!("? {}", q.iter().format(" "));

        input! {
            t: usize,
        }

        ts.push(t);
    }

    let mut graph = vec![Vec::new(); n];
    for i in 0..n {
        let j = (i + k) % n;
        graph[i].push(j);
        graph[j].push(i);
    }

    let mut seen = vec![false; n];
    seen[0] = true;
    dfs(0, &graph, &ts, &mut ans, &mut seen);

    let mut ok = true;
    for i in 0..n {
        let mut sum = 0;
        for j in 0..k {
            sum += ans[(i + j) % n];
        }

        let t = sum % 2;
        if t != ts[i] {
            ok = false;
            break;
        }
    }

    if !ok {
        ans = ans.iter().map(|t| 1 - t).collect();
    }

    println!("! {}", ans.iter().format(" "));
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, ts: &Vec<usize>, ans: &mut Vec<usize>, seen: &mut Vec<bool>) {
    let next = &graph[v];
    for &u in next {
        if seen[u] {
            continue;
        }

        seen[u] = true;
        if ts[u] == ts[v] {
            ans[u] = ans[v];
        } else {
            ans[u] = 1 - ans[v];
        }
        dfs(u, graph, ts, ans, seen);
    }
}