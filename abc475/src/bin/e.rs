use cp_library::data_structure::arbitrary_binary_trie::ArbitraryBinaryTrie;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: Chars,
        s: [Chars; n],
        q: usize,
    }

    let mut trie = ArbitraryBinaryTrie::new();
    let mut vs = vec![vec![false; k]; n];

    for i in 0..n {
        for j in 0..k {
            vs[i][j] = s[i][j] == t[j];
        }

        trie.insert(&vs[i]);
    }

    for _ in 0..q {
        input! {
            i: Usize1,
            j: Usize1,
        }

        trie.remove(&vs[i]);

        vs[i][j] = !vs[i][j];

        trie.insert(&vs[i]);

        let all_wrong = vs[i].iter().all(|&x| !x);

        let greater = trie.count_greater(&vs[i]);

        if !all_wrong && greater <= m {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}