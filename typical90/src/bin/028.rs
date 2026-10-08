use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut imos = vec![vec![0; 1000 + 1]; 1000 + 1];
    for _ in 0..n {
        input! {
            lx: usize,
            ly: usize,
            rx: usize,
            ry: usize,
        }

        imos[lx][ly] += 1;
        imos[rx][ly] -= 1;
        imos[lx][ry] -= 1;
        imos[rx][ry] += 1;
    }

    for i in 0..imos.len() {
        for j in 1..imos.len() {
            imos[i][j] += imos[i][j - 1];
        }
    }

    for j in 0..imos.len() {
        for i in 1..imos.len() {
            imos[i][j] += imos[i - 1][j];
        }
    }

    let mut v = vec![0; n + 1];
    for i in 0..imos.len() {
        for j in 0..imos.len() {
            v[imos[i][j]] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", v[i]);
    }
}
