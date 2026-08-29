use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut imos1 = vec![0; n];
    let mut imos2 = vec![0; n];

    for i in 0..n {
        input! {
            c: usize,
            p: usize,
        }
        if c == 1 {
            imos1[i] = p;
        } else {
            imos2[i] = p;
        }
    }

    for i in 1..n {
        imos1[i] += imos1[i - 1];
        imos2[i] += imos2[i - 1];
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }
        if l == 0 {
            println!("{} {}", imos1[r], imos2[r]);
        } else {
            println!("{} {}", (imos1[r] - imos1[l - 1]), (imos2[r] - imos2[l - 1]));
        }
    }
}
