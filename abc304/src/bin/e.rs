use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut dsu = Dsu::new(n);
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        dsu.merge(u, v);
    }

    input! {
        k: usize,
    }

    let mut set = HashSet::new();
    let mut ok = true;
    for _ in 0..k {
        input! {
            x: Usize1,
            y: Usize1
        }
        if dsu.same(x, y) {
            ok = false;
        }

        let x_leader = dsu.leader(x);
        let y_leader = dsu.leader(y);

        set.insert((x_leader, y_leader));
        set.insert((y_leader, x_leader));
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            p: Usize1,
            q: Usize1
        }
        let p_leader = dsu.leader(p);
        let q_leader = dsu.leader(q);
        
        if !ok || set.contains(&(p_leader, q_leader)) || set.contains(&(q_leader, p_leader)) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
