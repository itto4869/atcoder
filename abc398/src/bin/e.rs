use std::collections::HashSet;
use std::cmp::{min, max};
use proconio::{input, marker::Usize1};

// インタラクティブ問題では fastout は使わない（バッファリングで出力が遅れるため）
fn main() {
    input! {
        n: usize,
    }
    
    // 木の辺は N-1 本
    let mut edges = Vec::new();
    let mut graph = vec![Vec::new(); n];
    
    for _ in 0..n - 1 {
        input! {
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        graph[v].push(u);
        // 既存の辺を記録（小さい順）
        edges.push((min(u, v), max(u, v)));
    }

    // 彩色
    let mut color = vec![-1; n];
    color[0] = 0;
    dfs(0, &graph, &mut color);

    // バグ修正: 色の値ではなく、その色の「頂点インデックス」を集める
    let u_group: Vec<usize> = (0..n).filter(|&i| color[i] == 0).collect();
    let v_group: Vec<usize> = (0..n).filter(|&i| color[i] == 1).collect();

    // 可能なすべての二部マッチング辺を生成
    let mut set = HashSet::new();
    for &u in &u_group {
        for &v in &v_group {
            set.insert((min(u, v), max(u, v)));
        }
    }

    // ルール修正: 既に木として存在している辺は、候補から削除する
    for edge in edges {
        set.remove(&edge);
    }

    // 残りの引ける辺の数で先手後手を決定
    // set.len() がそのまま残り手数になる
    if set.len() % 2 == 0 {
        println!("Second");
    } else {
        println!("First");
        // 最初の一手
        make_move(&mut set);
    }

    // ゲームループ
    loop {
        // 相手の手を受け取る
        // 相手の入力は 1-based の可能性が高いので修正が必要かもしれないが
        // ここでは raw i64 で受け取って調整する
        input! {
            in_u: i64,
            in_v: i64,
        }

        // 終了条件
        if in_u == -1 && in_v == -1 {
            return;
        }

        // 相手の入力は1-indexと想定して0-indexに変換（問題によるので要確認）
        let u = (in_u - 1) as usize;
        let v = (in_v - 1) as usize;

        // 相手が使った辺を削除
        let edge = (min(u, v), max(u, v));
        set.remove(&edge);

        // 自分の手番
        make_move(&mut set);
    }
}

// 自分の手を打つ関数
fn make_move(set: &mut HashSet<(usize, usize)>) {
    // セットから適当に1つ取り出す（O(1)）
    // cloneして取らないとborrow checkerに怒られるため
    if let Some(&edge) = set.iter().next() {
        set.remove(&edge);
        // +1 して出力（1-index出力）
        println!("{} {}", edge.0 + 1, edge.1 + 1);
    } else {
        // もう打つ手がない（ここに来る前に相手が投了するか、ゲームが終わるはず）
        return;
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<i64>) {
    for &u in &graph[v] {
        if color[u] != -1 {
            continue;
        }
        color[u] = 1 - color[v];
        dfs(u, graph, color);
    }
}