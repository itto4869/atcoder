use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: i64,
        c: [i64; n],
        d: [i64; n - 1],
    }

    if d.iter().any(|&x| x > t) {
        println!("-1");
        return;
    }

    // 各町の位置
    let mut pos = vec![0i64; n];
    for i in 0..n - 1 {
        pos[i + 1] = pos[i] + d[i];
    }

    // 各町から見て、右側にある最初の安い町
    let mut next = vec![None; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in (0..n - 1).rev() {
        while let Some(&j) = stack.last() {
            if c[j] < c[i] {
                break;
            }
            stack.pop();
        }

        next[i] = stack.last().copied();
        stack.push(i);
    }

    let mut fuel = 0i64;
    let mut ans = 0i64;

    for i in 0..n - 1 {
        let remain = pos[n - 1] - pos[i];

        let target = match next[i] {
            Some(j) if pos[j] - pos[i] <= t => {
                pos[j] - pos[i]
            }
            _ => t.min(remain),
        };

        let buy = (target - fuel).max(0);

        fuel += buy;
        ans += buy * c[i];

        fuel -= d[i];
    }

    println!("{}", ans);
}