use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hw: [(usize, usize); n],
    }
    let mut s = Vec::with_capacity(n);
    for i in 0..n {
        s.push((hw[i].0, hw[i].1, i));
    }
    let mut l_v = s.clone();
    let mut r_v = s.clone();
    l_v.sort_unstable();
    l_v.reverse();
    r_v.sort_by(|a, b| {
        if a.1.cmp(&b.1) == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    r_v.reverse();

    let mut l_idx = 0;
    let mut r_idx = 0;
    let mut curr = (h, w);
    let mut ans = vec![(0, 0); n];
    while r_idx < n && l_idx < n && curr != (0, 0) {
        let (lh, lw, li) = l_v[l_idx];
        let (rh, rw, ri) = r_v[r_idx];
        if lh == curr.0 {
            ans[li] = (1, curr.1 - lw + 1);
            curr = (curr.0, curr.1 - lw);
            
            l_idx += 1;
            if ans[ri] != (0, 0) {
                    r_idx += 1;
                }
            else if rw == curr.1 {
                ans[ri] = (curr.0 - rh + 1, 1);
                curr = (curr.0 - rh, curr.1);
                
                r_idx += 1;
            } else {
                if ans[ri] != (0, 0) {
                    r_idx += 1;
                }
            }
        } else if rw == curr.1 {
            ans[ri] = (curr.0 - rh + 1, 1);
            curr = (curr.0 - rh, curr.1);
            
            r_idx += 1;

            if ans[li] != (0, 0) {
                    l_idx += 1;
                }
            else if lh == curr.0 {
                ans[li] = (1, curr.1 - lw + 1);
                curr = (curr.0, curr.1 - lw);

                l_idx += 1;
            } else {
                if ans[li] != (0, 0) {
                    l_idx += 1;
                }
            }
        } else {
            if ans[li] != (0, 0) {
                l_idx += 1;
            }

            if ans[ri] != (0, 0) {
                r_idx += 1;
            }
        }
    }

    for &u in &ans {
        println!("{} {}", u.0, u.1);
    }
}
