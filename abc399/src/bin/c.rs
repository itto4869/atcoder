use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1
        }
        if dsu.same(u, v) {
            ans += 1;
        } else {
            dsu.merge(u, v);
        }
    }

    println!("{}", ans);
}
