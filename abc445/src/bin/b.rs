use proconio::{fastout, input, marker::{Bytes, Chars}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut m = 0;
    for si in &s {
        m = m.max(si.len());
    }

    for si in &s {
        let k = (m - si.len()) / 2;
        let mut res = String::new();
        res.push_str(&".".repeat(k));
        let ssi: String = si.iter().collect();
        res.push_str(&ssi);
        res.push_str(&".".repeat(k));
        println!("{}", res);
    }
}
