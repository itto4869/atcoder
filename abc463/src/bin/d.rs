use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut lr: [(i64, i64); n],
    }

    lr.sort_by_key(|&(_, r)| r);

    let mut ok: i64 = 0;
    let mut ng: i64 = 1_000_000_001;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let mut pr: i64 = -1;
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