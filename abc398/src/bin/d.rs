use std::collections::HashSet;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: i64,
        c: i64,
        s: Bytes,
    }
    let mut fire = (0, 0);
    let mut takahashi = (r, c);
    let mut set = HashSet::new();
    set.insert(fire);

    let mut ans = String::with_capacity(n);
    for &c in &s {
        match c {
            b'N' => { 
                    fire.0 += 1;
                    takahashi.0 += 1;
                },
            b'W' => {
                    fire.1 += 1;
                    takahashi.1 += 1;
                },
            b'S' => {
                    fire.0 -= 1;
                    takahashi.0 -= 1;
                },
            b'E' => {
                    fire.1 -= 1;
                    takahashi.1 -= 1;
                },
            _ => unreachable!(),
        }
        set.insert(fire);
        if set.contains(&takahashi) {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }

    println!("{}", ans);
}
