use proconio::{fastout, input, marker::Usize1};
use std::ops::{Add, Sub, Neg};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        lrd: [(Usize1, Usize1, i64); m]
    }
    let mut dsu = WeightedUnionFind::new(n);
    for &(li, ri, di) in &lrd {
        dsu.unite(li, ri, di);
    }

    let mut ok = true;
    for &(li, ri, di) in &lrd {
        let diff = dsu.diff(li, ri).unwrap();
        if diff != di {
            ok = false;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[derive(Debug, Clone)]
pub struct WeightedUnionFind<T> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<T>, // 親ノードとの重みの差 (weight[p] - weight[x])
}

impl<T> WeightedUnionFind<T>
where
    T: Clone + Copy + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Default + PartialEq,
{
    pub fn new(n: usize) -> Self {
        WeightedUnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            diff_weight: vec![T::default(); n],
        }
    }

    // 根を探す（経路圧縮と同時に重みを更新）
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let root = self.find(self.parent[x]);
            // 再帰から戻る際に、親の重みを加算して更新
            let parent_weight = self.diff_weight[self.parent[x]]; 
            self.diff_weight[x] = self.diff_weight[x] + parent_weight;
            self.parent[x] = root;
            root
        }
    }

    // weight(y) - weight(x) = w となるように併合
    pub fn unite(&mut self, x: usize, y: usize, w: T) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            // 既に同じグループの場合、矛盾がないか確認する（オプション）
            // self.diff(x, y) == w
            return false;
        }

        // ランクによる併合（木の高さが低い方を高い方に繋ぐ）
        // 更新式: weight(y) - weight(x) = w
        // => (weight(root_y) + diff_y) - (weight(root_x) + diff_x) = w
        // => weight(root_y) - weight(root_x) = w + diff_x - diff_y
        let weight = w + self.diff_weight[x] - self.diff_weight[y];

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.diff_weight[root_x] = -weight;
        } else {
            self.parent[root_y] = root_x;
            self.diff_weight[root_y] = weight;
            if self.rank[root_x] == self.rank[root_y] {
                self.rank[root_x] += 1;
            }
        }
        true
    }

    // weight(y) - weight(x) を取得
    pub fn diff(&mut self, x: usize, y: usize) -> Option<T> {
        if self.find(x) != self.find(y) {
            None
        } else {
            Some(self.diff_weight[y] - self.diff_weight[x])
        }
    }
    
    // 同じグループに属するか
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}