use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut cs = vec![Vec::new(); n];
    for i in 0..n {
        for _ in (i + 1)..n {
            input! {
                ci: u64,
            }
            cs[i].push(ci);
        }
    }

    let mut ok = false;
    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                if cs[a][c - a - 1] > (cs[a][b - a - 1] + cs[b][c - b - 1]) {
                    ok = true;
                }
            }
        }
    }

    yes_no(ok);
}
