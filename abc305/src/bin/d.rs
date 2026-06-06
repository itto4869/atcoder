use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut fa = vec![0; n];
    for i in 1..n {
        if i % 2 == 0 {
            fa[i] = fa[i - 1] + a[i] - a[i - 1];
        } else {
            fa[i] = fa[i - 1];
        }
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", f(&a, &fa, r) - f(&a, &fa, l));
    }
}

fn f(a: &Vec<usize>, fa: &Vec<usize>, x: usize) -> usize {
    let j = a.partition_point(|&p| p < x).saturating_sub(1);
    fa[j] + (fa[j + 1] - fa[j]) / (a[j + 1] - a[j]) * (x - a[j])
}