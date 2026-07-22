use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
    }

    let bytes = s.as_bytes();
    let pattern = b"ATCODER";

    // ATCODER が始まる位置を列挙する
    let mut positions = Vec::new();

    let mut i = 0;
    while i + 7 <= n {
        if &bytes[i..i + 7] == pattern {
            positions.push(i);
        }
        i += 1;
    }

    if positions.is_empty() {
        println!("0");
        return;
    }

    // 連続して並んでいる ATCODER を塊にまとめる
    // (開始位置, 終了位置, 個数)
    let mut runs = Vec::new();

    let mut run_start = positions[0];
    let mut previous = positions[0];
    let mut run_length = 1;

    for &position in positions.iter().skip(1) {
        if position == previous + 7 {
            run_length += 1;
        } else {
            runs.push((run_start, previous + 7, run_length));

            run_start = position;
            run_length = 1;
        }

        previous = position;
    }

    runs.push((run_start, previous + 7, run_length));

    let mut edge_count = 0usize;
    let mut internal_runs = Vec::new();

    for &(left, right, length) in &runs {
        // 文字列全体が ATCODER の繰り返しの場合
        if left == 0 && right == n {
            let answer = if m >= length - 1 {
                length
            } else {
                m
            };

            println!("{}", answer);
            return;
        }

        if left == 0 || right == n {
            edge_count += length;
        } else {
            internal_runs.push(length);
        }
    }

    let mut remaining = m;
    let mut answer = 0usize;

    // 端にあるものは、1個につき切断1回
    let take_from_edge = edge_count.min(remaining);
    answer += take_from_edge;
    remaining -= take_from_edge;

    // 長い塊から使う
    internal_runs.sort_unstable_by(|a, b| b.cmp(a));

    for length in internal_runs {
        // 途中の塊から1個取るにも最低2回必要
        if remaining < 2 {
            break;
        }

        // t個取るにはt+1回必要
        let take = length.min(remaining - 1);

        answer += take;
        remaining -= take + 1;
    }

    println!("{}", answer);
}