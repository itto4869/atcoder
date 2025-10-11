use proconio::{fastout, input, marker::Usize1};
use ac_library::Dsu;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    let mut dsu = Dsu::new(n);
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        graph[v].push(u);
        dsu.merge(u, v);
    }

    let groups = dsu.groups();
    let mut ans = 0;
    for group in groups {
        let mut counts = Vec::new();
        for u in group {
            let mut color = vec![-1; n];
            color[u] = 0;
            paint_dfs(u, &graph, &mut color);

            let mut count = 0;

            for u in 0..n {
                let edges = &graph[u];
                if edges.is_empty() {
                    continue;
                }
                for v in edges {
                    if *v < u {
                        continue;
                    }
                    if color[u] == -1 || color[*v] == -1 {
                        continue;
                    }
                    if color[u] == color[*v] {
                        count += 1;
                    }
                }
            }
            counts.push(count);
        }
        let mut min_count = std::u64::MAX;
        for num in counts {
            min_count = min_count.min(num);
        }
        ans += min_count;
    }

    println!("{}", ans);

}

fn paint_dfs(v: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<i32>) {
    let edges = &graph[v];
    for u in edges {
        if color[*u] != -1 {
            continue;
        }
        color[*u] = 1 - color[v];
        paint_dfs(*u, graph, color);
    }
}

fn count_dfs(v: usize, graph: &Vec<Vec<usize>>, color: &Vec<i32>, seen: &mut Vec<bool>, mut ans: u64) -> u64 {
    let edges = &graph[v];
    for u in edges {
        if seen[*u] {
            continue;
        }
        seen[*u] = true;
        println!("v{}, u{}", v, u);
        if color[v] == color[*u] {
            ans += 1;
        }
        count_dfs(*u, graph, color, seen, ans);
    }
    ans
}