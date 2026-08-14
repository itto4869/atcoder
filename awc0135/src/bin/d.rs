use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
        v: u128,
        mut xd: [(u128, u128); n],
    }
    xd.sort_unstable();
    let mut l = xd[0].0.saturating_sub(v - 1);
    let mut r = xd[0].0 + (v - 1);
    for &(x, d) in &xd {
        let mut nl = x.saturating_sub(v - 1);
        let mut nr = x + (v - 1);
        if nl > r {
            println!("-1");
            return;
        }

        l = l.max(nl);
        r = r.min(nr);
    }
    let mut ok = *xd.iter().map(|(a, b)| b).max().unwrap();
    let mut ng = 0;
    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        let dist = (xd[0].1 + mid - 1) / mid;
        if dist > v {
            ng = mid;
            continue;
        }

        let mut l = xd[0].0.saturating_sub(v - dist);
        let mut r = xd[0].0 + (v - dist);
        let mut f = true;
        for &(x, d) in &xd {
            let dist = (d + mid - 1) / mid;
            if dist > v {
                f = false;
                break;
            }
            let nl = x.saturating_sub(v - dist);
            let nr = x + (v - dist);

            if nl > r {
                f = false;
                break;
            }

            l = l.max(nl);
            r = r.min(nr);
        }

        if !f {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    println!("{}", ok);
}
