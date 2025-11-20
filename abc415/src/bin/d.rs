use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64,
        m: usize,
        ab: [(u64, u64); m],
    }
    let mut ops: Vec<(u64, u64)> = ab
        .into_iter()
        .map(|(a, b)| (a - b, a))
        .collect();

    ops.sort_unstable();

    let mut ans = 0;

    let mut min_need_so_far = u64::MAX;

    for (cost, need) in ops {
        if need < min_need_so_far {
            min_need_so_far = need;

            if n >= need {
                let count = (n - need) / cost + 1;

                ans += count;
                n -= count * cost;
            }
        }
    }
    println!("{}", ans);
}
