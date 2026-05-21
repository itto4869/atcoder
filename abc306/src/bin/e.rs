use proconio::input;
use std::collections::BTreeMap;

// 多重集合の役割を果たす構造体を定義
struct TopKMultiSet {
    k: usize,
    x: BTreeMap<i64, usize>, // 上位K個の要素 <値, 出現回数>
    y: BTreeMap<i64, usize>, // それ以外の要素 <値, 出現回数>
    x_cnt: usize,            // Xに含まれる要素の総数（重複含む）
    y_cnt: usize,            // Yに含まれる要素の総数（重複含む）
    pub sum: i64,            // Xの要素の合計値
}

impl TopKMultiSet {
    // 初期化（K個をXに、N-K個をYに0として追加）
    fn new(k: usize, n: usize) -> Self {
        let mut ms = TopKMultiSet {
            k,
            x: BTreeMap::new(),
            y: BTreeMap::new(),
            x_cnt: 0,
            y_cnt: 0,
            sum: 0,
        };
        for _ in 0..k {
            ms.add_x(0);
        }
        for _ in k..n {
            ms.add_y(0);
        }
        ms
    }

    // Xに要素を追加
    fn add_x(&mut self, v: i64) {
        *self.x.entry(v).or_insert(0) += 1;
        self.x_cnt += 1;
        self.sum += v;
    }

    // Xから要素を1つ削除
    fn remove_x(&mut self, v: i64) {
        if let Some(count) = self.x.get_mut(&v) {
            *count -= 1;
            self.x_cnt -= 1;
            self.sum -= v;
            if *count == 0 {
                self.x.remove(&v); // カウントが0になったらキーごと削除
            }
        }
    }

    // Yに要素を追加
    fn add_y(&mut self, v: i64) {
        *self.y.entry(v).or_insert(0) += 1;
        self.y_cnt += 1;
    }

    // Yから要素を1つ削除
    fn remove_y(&mut self, v: i64) {
        if let Some(count) = self.y.get_mut(&v) {
            *count -= 1;
            self.y_cnt -= 1;
            if *count == 0 {
                self.y.remove(&v); // カウントが0になったらキーごと削除
            }
        }
    }

    // XとYの要素を適切な状態に保つ（解説のbalance関数に相当）
    fn balance(&mut self) {
        // Xの要素数がKに満たない場合、Yの最大値をXに移す
        while self.x_cnt < self.k && self.y_cnt > 0 {
            // last_key_value() で BTreeMap の最大値を取得
            let max_y = *self.y.last_key_value().unwrap().0;
            self.remove_y(max_y);
            self.add_x(max_y);
        }

        if self.x_cnt == 0 || self.y_cnt == 0 {
            return;
        }

        // Xの最小値 < Yの最大値 の場合、それらを入れ替える
        loop {
            let min_x = *self.x.first_key_value().unwrap().0;
            let max_y = *self.y.last_key_value().unwrap().0;

            if min_x >= max_y {
                break;
            }

            self.remove_x(min_x);
            self.remove_y(max_y);
            self.add_x(max_y);
            self.add_y(min_x);
        }
    }

    // 要素vを追加し、バランスを取る
    fn add(&mut self, v: i64) {
        self.add_y(v);
        self.balance();
    }

    // 要素vを削除し、バランスを取る
    fn erase(&mut self, v: i64) {
        if self.x.contains_key(&v) {
            self.remove_x(v);
        } else {
            self.remove_y(v);
        }
        self.balance();
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
    }

    let mut a = vec![0i64; n];
    let mut top_k = TopKMultiSet::new(k, n);

    for _ in 0..q {
        input! {
            p: usize,
            w: i64,
        }
        let p = p - 1; // 0-indexed に変換

        // 解説の手順通り、追加してから削除する
        top_k.add(w);
        top_k.erase(a[p]);
        a[p] = w;

        // クエリごとにXの合計値を出力
        println!("{}", top_k.sum);
    }
}