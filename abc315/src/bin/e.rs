use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    // グラフの構築: graph[u] は u を読むために必要な本のリスト
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        input! {
            c: usize,
            p: [Usize1; c],
        }
        graph[i] = p;
    }

    let mut visited = vec![false; n];
    let mut ans = vec![];

    // 深さ優先探索 (DFS)
    fn dfs(u: usize, graph: &[Vec<usize>], visited: &mut [bool], ans: &mut Vec<usize>) {
        visited[u] = true;
        for &v in &graph[u] {
            if !visited[v] {
                dfs(v, graph, visited, ans);
            }
        }
        // 帰りがけ順でリストに追加
        ans.push(u);
    }

    // 本1 (インデックス0) から探索開始
    dfs(0, &graph, &mut visited, &mut ans);

    // 本1自体は出力に含めないため、末尾を削除
    ans.pop();

    println!("{}", ans.iter().map(|&x| x + 1).format(" "));
}