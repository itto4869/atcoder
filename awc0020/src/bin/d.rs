use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
    }
    let mut lr = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            x: usize,
            r: usize,
        }
        lr.push((x.saturating_sub(r), x + r));
    }

    let mut a = 0;
    let mut b = 0;

    lr.sort_unstable();
    let mut idx = 0;
    let mut ok = true;
    while idx < n {
        let (na, nb) = lr[idx];
        if b < na {
            ok = false;
            break;
        }

        a = na;
        b = b.max(nb);

        idx += 1;

        if b >= l {
            break;
        }
    }

    if b < l {
        ok = false;
    }

    yes_no(ok);
}
