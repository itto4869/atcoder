use cp_data_structure::implicit_treap::ImplicitTreap;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m]
    }
    let mut treap = ImplicitTreap::new();
    for i in 0..n {
        treap.push_back(i + 1);
    }

    if m == 0 {
        println!("{}", treap.into_vec().iter().format(" "));
        return;
    }
    let mut l = a[0];
    let mut r = a[0] + 1;
    for i in 1..m {
        if a[i] == (a[i - 1] + 1) {
            r += 1;
        } else {
            treap.reverse(l, r + 1);
            l = a[i];
            r = a[i] + 1;
        }
    }

    if l != r {
        treap.reverse(l, r + 1);
    }

    let ans = treap.into_vec();
    println!("{}", ans.iter().format(" "));
}
