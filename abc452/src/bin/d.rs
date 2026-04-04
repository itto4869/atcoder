use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();

    // next[i][c] = i以上で文字cが最初に現れる位置。なければ n
    let mut next = vec![[n; 26]; n + 1];

    for i in (0..n).rev() {
        next[i] = next[i + 1];
        let c = (s[i] as u8 - b'a') as usize;
        next[i][c] = i;
    }

    let total = n as u64 * (n as u64 + 1) / 2;
    let mut contain = 0u64;

    for l in 0..n {
        let mut pos = l;
        let mut ok = true;

        for &ch in &t {
            let c = (ch as u8 - b'a') as usize;
            let p = next[pos][c];
            if p == n {
                ok = false;
                break;
            }
            pos = p + 1;
        }

        if ok {
            let min_r = pos - 1;
            contain += (n - min_r) as u64;
        }
    }

    let ans = total - contain;
    println!("{}", ans);
}