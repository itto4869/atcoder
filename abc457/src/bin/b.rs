use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            l: usize,
        }
        for _ in 0..l {
            input! {
                a: usize,
            }
            v[i].push(a);
        }
    }

    input! {
        x: Usize1,
        y: Usize1,
    }
    println!("{}", v[x][y]);
}
