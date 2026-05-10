use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    }

    let set: HashSet<Vec<u8>> = s
        .into_iter()
        .map(|x| x.into_bytes())
        .collect();

    // 2^M 個すべてが禁止されている場合だけ No
    // N <= 2 * 10^4 なので、M が大きい場合は必ず 2^M > N
    if m <= 20 && n == (1usize << m) {
        println!("No");
        return;
    }

    for x in 0..=n {
        // x を M 桁の2進数文字列にする
        let t = format!("{:0width$b}", x, width = m);
        let t_bytes = t.as_bytes();

        // T の反転を作る
        let rev: Vec<u8> = t_bytes
            .iter()
            .map(|&c| if c == b'0' { b'1' } else { b'0' })
            .collect();

        // 反転(T) が S に含まれていなければ、T は条件を満たす
        if !set.contains(&rev) {
            println!("Yes");
            println!("{}", t);
            return;
        }
    }

    // 通常ここには来ない
    println!("No");
}