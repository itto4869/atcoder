use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        _: usize,
        k: u64,
        s: Bytes,
        t: Bytes,
    }
    let mut cnt = 0u64;
    for (si, ti) in s.into_iter().zip(t) {
        if si != ti {
            cnt += 1;
        }
    }

    let ans = cnt.saturating_sub(k);
    println!("{}", ans);
}
