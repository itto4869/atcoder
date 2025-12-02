use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut imos = vec![vec![0i64; 1001]; 1001];
    for _ in 0..n {
        input! {
            lx: usize,
            ly: usize,
            rx: Usize1,
            ry: Usize1,
        }
        imos[ly][lx] += 1;
        imos[ly][rx + 1] -= 1;
        imos[ry + 1][lx] -= 1;
        imos[ry + 1][rx + 1] += 1;
    }

    for i in 0..=1000 {
        for j in 1..=1000 {
            imos[i][j] += imos[i][j - 1];
        }
    }

    for j in 0..=1000 {
        for i in 1..=1000 {
            imos[i][j] += imos[i - 1][j];
        }
    }

    let mut cnt = vec![0; n + 1];
    for i in 0..=1000 {
        for j in 0..=1000 {
            let k = imos[i][j];
            cnt[k as usize] += 1;
        }
    }

    for &k in cnt.iter().skip(1) {
        println!("{}", k);
    }
}
