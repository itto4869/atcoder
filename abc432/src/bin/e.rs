use proconio::{fastout, input};

#[derive(Clone, Debug)]
struct SegTree {
    size: usize,
    data: Vec<i64>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let mut size = 1usize;
        while size < n {
            size <<= 1;
        }
        Self {
            size,
            data: vec![0; 2 * size],
        }
    }

    // 点加算: a[idx] += val
    fn add(&mut self, mut idx: usize, val: i64) {
        idx += self.size;
        self.data[idx] += val;
        while idx > 1 {
            idx >>= 1;
            self.data[idx] = self.data[idx << 1] + self.data[idx << 1 | 1];
        }
    }

    // 区間和 [l, r)
    fn sum(&self, mut l: usize, mut r: usize) -> i64 {
        let mut res = 0i64;
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                res += self.data[l];
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res += self.data[r];
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
    }

    const MAXV: usize = 500_001; // 値は 0..=500000

    let mut seg_cnt = SegTree::new(MAXV); // 個数
    let mut seg_sum = SegTree::new(MAXV); // 値の総和
    for &v in &a {
        let idx = v as usize;
        seg_cnt.add(idx, 1);
        seg_sum.add(idx, v);
    }

    let total_n = n as i64;

    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            // 1 x y : A[x] = y
            input! { x: usize, y: i64 }
            let pos = x - 1;
            let old = a[pos];
            if old == y {
                continue;
            }

            // 古い値を削除
            let old_idx = old as usize;
            seg_cnt.add(old_idx, -1);
            seg_sum.add(old_idx, -old);

            // 新しい値を追加
            let new_idx = y as usize;
            seg_cnt.add(new_idx, 1);
            seg_sum.add(new_idx, y);

            a[pos] = y;
        } else {
            // 2 l r : sum max(l, min(r, Ai))
            input! { l: i64, r: i64 }

            // ★ l > r の場合はすべて l になる
            if l > r {
                println!("{}", l * total_n);
                continue;
            }

            let l_us = l as usize;
            let r_us = r as usize;

            // cnt_lt_l = Σ_{v<l} cnt[v]
            let cnt_lt_l = if l_us == 0 {
                0
            } else {
                seg_cnt.sum(0, l_us)
            };

            // cnt_le_r = Σ_{v≤r} cnt[v]
            let cnt_le_r = seg_cnt.sum(0, r_us + 1);
            let cnt_gt_r = total_n - cnt_le_r;

            // sum_between = Σ_{l≤v≤r} v * cnt[v]
            let sum_between = seg_sum.sum(l_us, r_us + 1);

            let ans = l * cnt_lt_l + sum_between + r * cnt_gt_r;
            println!("{}", ans);
        }
    }
}
