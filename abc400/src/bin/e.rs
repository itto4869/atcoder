use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let n = 1000001;
    let mut v = vec![0; n];
    for i in 2..n {
        if v[i] == 0 {
            for j in (i..n).step_by(i) {
                v[j] += 1;
            }
        }
    }

    let mut best_k = vec![0; n];
    let mut current_best = 0;

    for i in 1..n {
        if v[i] == 2 {
            current_best = i;
        }
        best_k[i] = current_best;
    }

    for _ in 0..q {
        input! {
            a: usize,
        }

        let r = a.isqrt();
        let k = best_k[r];

        println!("{}", k * k);
    }
}
