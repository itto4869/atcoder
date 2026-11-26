use cp_data_structure::trie::Trie;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut trie = Trie::new();
    for x in &s {
        trie.insert(x);
    }

    for x in s {
        let cnts = trie.prefix_counts(&x);
        let res = cnts.partition_point(|&p| p >= 2);
        println!("{}", res);
    }
}