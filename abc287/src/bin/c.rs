use cp_utils::yes_no;
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
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut v = 0;
    for i in 0..n {
        if graph[i].len() == 1 {
            v = i;
            break;
        }
    }

    
    let mut seen = vec![false; n];
    seen[v] = true;
    let mut ok = true;
    dfs(v, &graph, &mut seen, &mut ok);

    ok &= seen.iter().all(|&x| x);
    yes_no!(ok);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, ok: &mut bool) {
    let next = &graph[v];
    if next.len() > 2 {
        return;
    }
    let mut cnt = 0;
    for &u in next {
        if seen[u] {
            cnt += 1;
            continue;
        }

        seen[u] = true;
        dfs(u, graph, seen, ok);
    }

    if cnt == 2 {
        *ok = false;
    }
}