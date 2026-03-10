use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u8; n],
    }

    let mut end = vec![0u8; n + 1]; // ここで効果が終わる反転の数(偶奇だけ見ればよい)
    let mut flip = 0u8;             // 現在位置に効いている反転回数の偶奇
    let mut ans = 0usize;

    for i in 0..n {
        flip ^= end[i];

        let cur = a[i] ^ flip;
        if cur == 1 {
            // ここで反転を始めるしかない
            if i + k > n {
                println!("-1");
                return;
            }
            ans += 1;
            flip ^= 1;
            end[i + k] ^= 1;
        }
    }

    println!("{}", ans);
}