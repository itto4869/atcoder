use ac_library::Dsu;
use cp_utils::yes_no;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut degree = vec![0; n];
    let mut dsu = Dsu::new(n);
    for _ in 0..m {
        input! {
            v: Usize1,
            u: Usize1,
        }
        dsu.merge(u, v);
        degree[u] += 1;
        degree[v] += 1;
    }

    let mut ok = true;
    for group in dsu.groups() {
        let n = group.len();
        let mut cnt = 0;
        for v in group {
            cnt += degree[v];
        }

        if (2 * n) != cnt {
            ok = false;
        }
    }

    yes_no!(ok);
}
