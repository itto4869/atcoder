use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut ans = 0;
    for i in 0..m {
        let mut dsu = Dsu::new(n);
        for j in 0..ab.len() {
            if i == j {
                dsu.merge(ab[j].0, ab[j].0);
                dsu.merge(ab[j].1, ab[j].1);
                continue;
            }
            dsu.merge(ab[j].0, ab[j].1);
        }
        if dsu.groups().len() != 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}