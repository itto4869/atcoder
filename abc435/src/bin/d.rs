use ac_library::Dsu;
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
            x: Usize1,
            y: Usize1,
        }
        graph[y].push(x);
    }
    
    input! {
        q: usize,
    }

    let mut reach = vec![false; n];
    for _ in 0..q {
        input! {
            c: u64,
        }

        if c == 1 {
            input! {
                v: Usize1,
            }
            dfs(v, &graph, &mut reach);
        } else {
            input! {
                v: Usize1,
            }
            if reach[v] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, reach: &mut Vec<bool>) {
    let next = &graph[v];
    for &u in next {
        if !reach[u] {
            reach[u] = true;
            dfs(u, graph, reach);
        } else {
            continue;
        }
    }
}