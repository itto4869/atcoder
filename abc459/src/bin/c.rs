use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = vec![0; n];
    let mut cnt = vec![Vec::new(); 3 * 10usize.pow(6)];
    let mut d = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
            }
            v[x] += 1;
            cnt[v[x]].push(x);
            if cnt[1 + d].len() == n {
                d += 1;
            }
        } else {
            input! {
                y: usize,
            }
            let res = cnt[y + d].len();
            println!("{}", res);
        }
    }
}
