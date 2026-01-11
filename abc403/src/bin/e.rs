use proconio::{input, marker::Chars};

// トライ木のノード定義
#[derive(Clone, Default)]
struct Node {
    children: [Option<usize>; 26], // 子ノードへのインデックス (a-z)
    cnt: usize,                    // このノードで終わるYの個数
    is_x: bool,                    // このノードで終わるXが存在するか
    sum: usize,                    // この部分木に含まれる有効なYの総数
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            // 最初にルートノード(index 0)を入れておく
            nodes: vec![Node::default()],
        }
    }

    // 新しいノードを作成し、そのインデックスを返す
    fn new_node(&mut self) -> usize {
        self.nodes.push(Node::default());
        self.nodes.len() - 1
    }

    // 文字 'a'-'z' を 0-25 に変換
    fn char_to_idx(c: char) -> usize {
        (c as u8 - b'a') as usize
    }

    // X に文字列を追加 (Type 1)
    fn insert_x(&mut self, s: &[char]) {
        let mut node_idx = 0;
        let mut path = vec![0]; // 更新用に通った道順を記録

        for &c in s {
            let c_idx = Self::char_to_idx(c);
            // 子ノードがなければ作成
            if self.nodes[node_idx].children[c_idx].is_none() {
                let new_idx = self.new_node();
                self.nodes[node_idx].children[c_idx] = Some(new_idx);
            }
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
            path.push(node_idx);
        }

        // Xの終端フラグを立てる
        self.nodes[node_idx].is_x = true;

        // 下から順に sum を更新
        self.update_path(&path);
    }

    // Y に文字列を追加 (Type 2)
    fn insert_y(&mut self, s: &[char]) {
        let mut node_idx = 0;
        let mut path = vec![0];

        for &c in s {
            let c_idx = Self::char_to_idx(c);
            if self.nodes[node_idx].children[c_idx].is_none() {
                let new_idx = self.new_node();
                self.nodes[node_idx].children[c_idx] = Some(new_idx);
            }
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
            path.push(node_idx);
        }

        // Yの個数を増やす
        self.nodes[node_idx].cnt += 1;

        // 下から順に sum を更新
        self.update_path(&path);
    }

    // パス上のノードの sum を下から上に再計算する
    fn update_path(&mut self, path: &[usize]) {
        // rev() で逆順（深い方からルートへ）
        for &idx in path.iter().rev() {
            // is_x が true なら、このノード以下は全て無効 -> sum = 0
            if self.nodes[idx].is_x {
                self.nodes[idx].sum = 0;
            } else {
                // そうでなければ、自身のcnt + 子ノードたちのsum
                let mut current_sum = self.nodes[idx].cnt;
                for child_opt in self.nodes[idx].children.iter() {
                    if let Some(child_idx) = child_opt {
                        current_sum += self.nodes[*child_idx].sum;
                    }
                }
                self.nodes[idx].sum = current_sum;
            }
        }
    }

    // 現在の答え（ルートのsum）を取得
    fn get_ans(&self) -> usize {
        self.nodes[0].sum
    }
}

fn main() {
    input! {
        q: usize,
        queries: [(u8, Chars); q],
    }

    let mut trie = Trie::new();

    for (t, s) in queries {
        if t == 1 {
            trie.insert_x(&s);
        } else {
            trie.insert_y(&s);
        }
        println!("{}", trie.get_ans());
    }
}