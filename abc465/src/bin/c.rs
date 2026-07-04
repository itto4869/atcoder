use cp_library::data_structure::implicit_treap::ImplicitTreap;
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut treap: ImplicitTreap<_> = (1..=n).collect();
    for idx in 0..n {
        let c = s[idx];
        if c == 'o' {
            treap.reverse(0, idx + 1);
        }
    }

    let ans = treap.to_vec();
    println!("{}", ans.iter().format(" "));
}
