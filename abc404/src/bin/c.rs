use cp_library::utils::yes_no;
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
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut ok = dfs(0, &graph, &mut seen, true);
    ok = ok & seen.iter().all(|&x| x == true);
    yes_no(ok);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, ok: bool) -> bool {
    let next = &graph[v];
    let mut ok = ok;
    if next.len() != 2 {
        return false;
    }
    for &u in next {
        if !seen[u] {
            seen[u] = true;
            ok = dfs(u, graph, seen, ok);
        }
    }
    ok
}