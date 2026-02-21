use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    // 頂点数は M * M
    let n_states = m * m;

    // 1. 逆辺のグラフを構築する
    // rev_graph[遷移先の頂点] = [遷移元の頂点1, 遷移元の頂点2, ...]
    let mut rev_graph = vec![Vec::new(); n_states];

    for x in 0..m {
        for y in 0..m {
            // 次の値 z を計算
            let z = (a * y + b * x) % m;
            
            // 状態 (x, y) を1つの整数(頂点ID)に変換
            let u = x * m + y; // 遷移元
            let v = y * m + z; // 遷移先
            
            // 矢印を逆向きにしてグラフに保存（遷移先 v から 遷移元 u へたどれるようにする）
            rev_graph[v].push(u);
        }
    }

    // 各頂点に到達したか（=ブラックホールに吸い込まれる運命か）を記録する配列
    let mut visited = vec![false; n_states];
    let mut queue = VecDeque::new();

    // 2. ブラックホール（0を含む状態）を探索のスタート地点としてキューに入れる
    // 状態 (0, t) は、要素に 0 が含まれることを意味する
    for t in 0..m {
        let start_node = 0 * m + t;
        if !visited[start_node] {
            visited[start_node] = true;
            queue.push_back(start_node);
        }
    }

    // 3. BFS（幅優先探索）で道を逆走する
    while let Some(v) = queue.pop_front() {
        // v に向かってくる遷移元 u をすべて確認する
        for &u in &rev_graph[v] {
            // まだ訪れていない（ブラックホール行きが確定していない）場合
            if !visited[u] {
                visited[u] = true; // ブラックホール行き確定
                queue.push_back(u); // さらにその手前の状態を探すためにキューに入れる
            }
        }
    }

    // 4. 一度も訪問されなかった（= どうやっても0に到達しない）安全な初期値の数を数える
    let mut safe_count = 0;
    for x in 0..m {
        for y in 0..m {
            let u = x * m + y;
            if !visited[u] {
                safe_count += 1;
            }
        }
    }

    println!("{}", safe_count);
}