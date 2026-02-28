use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for i in 0..s.len() {
        if s[i] == b'A' {
            a.push(i);
        } else if s[i] == b'B' {
            b.push(i);
        } else {
            c.push(i);
        }
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    let mut ans = 0;
    while i < a.len() && j < b.len() && k < c.len() {
        if a[i] < b[j] && b[j] < c[k] {
            ans += 1;
            i += 1;
            j += 1;
            k += 1;
            continue;
        }

        if a[i] > b[j] {
            j += 1;
        }

        if b[j] > c[k] {
            k += 1;
        }
    }

    println!("{}", ans);
}
