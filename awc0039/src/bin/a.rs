use cp_library::utils::yes_no;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        pc: [(Usize1, char); m],
    }
    for _ in 0..q {
        input! {
            t: Chars,
        }
        let mut ok = true;
        for &(p, c) in &pc {
            if t[p] != c {
                ok = false;
                break;
            }
        }

        yes_no(ok);
    }
}
