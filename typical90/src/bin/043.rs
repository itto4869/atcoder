use std::collections::VecDeque;
use proconio::{input, marker::{Bytes, Usize1}};

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: Usize1, cs: Usize1, // 0-indexedに変換
        rt: Usize1, ct: Usize1, // 0-indexedに変換
        s: [Bytes; h],
    }

    // dist[行][列][現在向いている方向] = 方向転換回数の最小値
    // 方向: 0:上, 1:右, 2:下, 3:左
    const INF: u32 = 1_000_000_000;
    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    let mut deque = VecDeque::new();

    // 方向の定義 (dr, dc)
    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    // 【重要】スタート地点の初期化処理
    // スタート地点から「最初の1歩」は方向転換としてカウントしないため、
    // スタートの上下左右にある有効なマスを「コスト0」としてキューに入れます。
    for dir in 0..4 {
        let nr = rs as isize + dy[dir];
        let nc = cs as isize + dx[dir];

        // 範囲内かつ壁でない場合
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            let nr = nr as usize;
            let nc = nc as usize;
            if s[nr][nc] == b'.' {
                dist[nr][nc][dir] = 0;
                deque.push_back((nr, nc, dir));
            }
        }
    }

    // 0-1 BFS
    while let Some((r, c, d)) = deque.pop_front() {
        // もし記録されているコストより大きいコストで取り出されたら無視（枝刈り）
        if dist[r][c][d] < dist[r][c][d] { 
            continue; 
        }

        // 次の移動先を探索（4方向）
        for next_dir in 0..4 {
            let nr = r as isize + dy[next_dir];
            let nc = c as isize + dx[next_dir];

            // 範囲外ならスキップ
            if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            // 壁ならスキップ
            if s[nr][nc] == b'#' {
                continue;
            }

            // 【重要】コスト計算
            // 直進（向きが同じ）なら +0
            // カーブ（向きが違う）なら +1
            let cost = if d == next_dir { 0 } else { 1 };
            
            if dist[nr][nc][next_dir] > dist[r][c][d] + cost {
                dist[nr][nc][next_dir] = dist[r][c][d] + cost;

                // 0-1 BFSのルールに従ってpush
                if cost == 0 {
                    deque.push_front((nr, nc, next_dir)); // コスト0は先頭へ
                } else {
                    deque.push_back((nr, nc, next_dir));  // コスト1は末尾へ
                }
            }
        }
    }

    // ゴール地点の4方向の中で最小のものが答え
    let mut ans = INF;
    for i in 0..4 {
        ans = ans.min(dist[rt][ct][i]);
    }

    println!("{}", ans);
}