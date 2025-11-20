use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut s: Bytes,
        }
        s.insert(0, b'0');
        let mut ok = vec![0; 1 << n];
        ok[0] = 1;
        for i in 0..(1 << n) {
            if ok[i] == 0 {
                continue;
            }
            for j in 0..n {
                if i & (1 << j) > 0 {
                    continue;
                }
                let next = i | 1 << j;
                if s[next] == b'0' {
                    ok[next] = 1;
                }
            }
        }
        if ok[(1 << n) - 1] == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
