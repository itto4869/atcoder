use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ok = 0;
    let mut ng = 1_000_000_000;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut pr = 0;
        let mut cnt = 0;
        for &(l, r) in &lr {
            if l > pr {
                pr = r + mid - 1;
                cnt += 1;
            }
        }

        if cnt >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if ok == 0 {
        println!("-1");
    } else {
        println!("{}", ok);
    }
}
