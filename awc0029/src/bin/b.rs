use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut v: [u64; n],
    }
    for _ in 0..q {
        input! {
            f: u64,
        }
        if f == 1 {
            input! {
                a: Usize1,
                b: Usize1
            }
            v[b] += v[a];
            v[a] = 0;
        } else {
            input! {
                c: Usize1,
            }

            let res = v[c];
            println!("{}", res);
        }
    }
}
