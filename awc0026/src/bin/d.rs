use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        lr: [(i64, i64); n],
    }

    let mut xs = Vec::with_capacity(2 * n);
    for &(l, r) in &lr {
        xs.push(l);
        xs.push(r);
    }
    xs.sort_unstable();
    xs.dedup();

    let idx = |x: i64, xs: &Vec<i64>| -> usize {
        xs.binary_search(&x).unwrap()
    };

    let mut diff = vec![0i32; xs.len() + 1];
    for &(l, r) in &lr {
        diff[idx(l, &xs)] += 1;
        diff[idx(r, &xs)] -= 1;
    }

    let mut cur = 0i32;
    let mut ans = 0i64;
    for i in 0..xs.len() - 1 {
        cur += diff[i];
        if cur >= k {
            ans += xs[i + 1] - xs[i];
        }
    }

    println!("{}", ans);
}