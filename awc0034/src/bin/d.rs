use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut lr: [(u64, u64); n],
    }
    lr.sort_by(|a, b| a.1.cmp(&b.1));
    let mut cr = 0;

    let mut ans = 0;
    for (l, r) in lr {
        if l < cr {
            continue;
        } else {
            cr = r;
            ans += 1;
        }
    }

    println!("{}", ans);
}
