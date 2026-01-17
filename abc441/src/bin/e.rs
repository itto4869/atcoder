use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut p = Vec::new();
    let mut cnt = 0isize;
    p.push(n);
    for &c in &s {
        if c == b'A' {
            cnt += 1;
        } else if c == b'B' {
            cnt -= 1;
        }

        p.push((cnt + n as isize) as usize);
    }

    let mut bit = FenwickTree::new(2 * n + 1, 0usize);
    let mut ans = 0;
    for i in 0..=n {
        let pi = p[i];
        let k = bit.sum(0..pi);
        ans += k;
        bit.add(pi, 1);
    }

    println!("{}", ans);
}
