use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
    }
    for _ in 0..q {
        input! {
            t: usize,
            k: Usize1,
        }
        println!("{}", f(t, k, &s));
    }
}

fn g(s: char, r: usize) -> char {
    return ('A' as usize + (s as usize - 'A' as usize + r) % 3) as u8 as char
}

fn f(t: usize, k: usize, s: &Vec<char>) -> char {
    if t == 0 {
        return s[k];
    } else if k == 0 {
        return g(s[0], t);
    } else {
        return g(f(t - 1, k / 2, s), k % 2 + 1);
    }
}