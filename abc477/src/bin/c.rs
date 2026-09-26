use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        q: usize,
        s: Chars,
        t: Chars,
    }
    if s.len() < t.len() {
        for _ in 0..q {
            input! {
                l: Usize1,
                r: Usize1
            }

            println!("No");
        }
        return;
    }
    let mut v = vec![s.len(); s.len()];
    let mut r = s.len();
    for i in (0..(s.len() - t.len() + 1)).rev() {
        let mut ok = true;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                ok = false;
                break;
            }
        }

        if ok {
            r = i + t.len() - 1;
        }

        v[i] = r;
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1
        }

        if v[l] <= r {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
