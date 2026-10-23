use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    for _ in 0..h {
        input! {
            s: Chars,
        }
        let res = s.into_iter().filter(|&c| c == '.').count();
        println!("{}", res);
    }
}
