use std::io::{self, Read};
use std::cmp::Ordering;

struct Monster {
    id: usize,
    x: i64,
    y: i64,
}

// 領域（象限）を判定する関数
// 下半分（第3,4象限 + 正のX軸）を0、上半分（第1,2象限 + 負のX軸）を1とする
fn get_region(x: i64, y: i64) -> i32 {
    if y < 0 || (y == 0 && x > 0) {
        0
    } else {
        1
    }
}

// 外積計算 (A x B)
fn cross_product(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    x1 * y2 - x2 * y1
}

fn main() {
    // --- 高速入力の準備 ---
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut monsters = Vec::with_capacity(n);
    for i in 0..n {
        let x: i64 = iter.next().unwrap().parse().unwrap();
        let y: i64 = iter.next().unwrap().parse().unwrap();
        monsters.push(Monster { id: i, x, y });
    }

    // --- 偏角ソート (反時計回り) ---
    monsters.sort_by(|a, b| {
        let region_a = get_region(a.x, a.y);
        let region_b = get_region(b.x, b.y);

        if region_a != region_b {
            return region_a.cmp(&region_b);
        }
        
        // 同じ領域なら外積で比較
        let cp = cross_product(a.x, a.y, b.x, b.y);
        if cp > 0 {
            // a -> b が反時計回り = aの方が角度が小さい
            Ordering::Less
        } else if cp < 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // 時計回りに処理したいので逆順にする
    monsters.reverse();

    // --- 同じ方向をまとめる & マッピング作成 ---
    // mapping[元のID] = グループID
    let mut mapping = vec![0; n];
    // group_counts[グループID] = その方向のモンスター数
    let mut group_counts = Vec::new();

    if n > 0 {
        let mut current_group_idx = 0;
        let mut current_count = 1;
        
        // 最初の要素を処理
        mapping[monsters[0].id] = 0;

        for i in 1..n {
            let prev = &monsters[i-1];
            let curr = &monsters[i];
            
            // 同一方向判定: 領域が同じ かつ 外積が0
            let same_region = get_region(prev.x, prev.y) == get_region(curr.x, curr.y);
            let same_dir = same_region && cross_product(prev.x, prev.y, curr.x, curr.y) == 0;

            if same_dir {
                current_count += 1;
            } else {
                group_counts.push(current_count);
                current_count = 1;
                current_group_idx += 1;
            }
            mapping[curr.id] = current_group_idx;
        }
        // 最後のグループを追加
        group_counts.push(current_count);
    }

    // --- 累積和の作成 ---
    let m = group_counts.len();
    let mut s = vec![0_i64; m + 1];
    for i in 0..m {
        s[i + 1] = s[i] + group_counts[i];
    }

    // --- クエリ処理 ---
    // 出力バッファリングのために String にまとめるか BufWriter を使う
    // ここでは単純に println! を使いますが、大量出力時は BufWriter 推奨
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    use std::io::Write;

    for _ in 0..q {
        let a_raw: usize = iter.next().unwrap().parse().unwrap();
        let b_raw: usize = iter.next().unwrap().parse().unwrap();
        
        // 0-indexed に変換
        let a_idx = mapping[a_raw - 1];
        let b_idx = mapping[b_raw - 1];

        let ans = if a_idx <= b_idx {
            s[b_idx + 1] - s[a_idx]
        } else {
            // 円環をまたぐ場合: [a, end] + [start, b]
            (s[m] - s[a_idx]) + s[b_idx + 1]
        };

        writeln!(out, "{}", ans).unwrap();
    }
}