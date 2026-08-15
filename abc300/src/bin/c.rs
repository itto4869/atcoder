use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut sn = vec![0; h.min(w)];
    for n in 1..=(h.min(w)) {
        for a in n..(h - n) {
            for b in n..(w - n) {
                let mut ok = true;
                if c[a][b] != '#' {
                    ok = false;
                    continue;
                }

                for d in 1..=n {
                    if (c[a + d][b + d] != '#') || (c[a + d][b - d] != '#') || (c[a - d][b + d] != '#') || (c[a - d][b - d] != '#') {
                        ok = false;
                        break;
                    }
                }

                if ((a + n + 1) < h) && ((b + n + 1) < w) && (c[a + n + 1][b + n + 1] == '#')
                    && ((b >= (n + 1))) && (c[a + n + 1][b - n - 1] == '#')
                    && (a >= (n + 1)) && (c[a - n - 1][b + n + 1] == '#')
                    && (c[a - n - 1][b - n - 1] == '#') {
                        ok = false;
                    }
                
                if ok {
                    sn[n - 1] += 1;
                }
            }
        }
    }

    println!("{}", sn.iter().format(" "));
}
