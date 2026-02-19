use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut imos = vec![vec![0; 102]; 102];
    for _ in 0..n {
        input! {
            a: usize,
            b: Usize1,
            c: usize,
            d: Usize1,
        }
        imos[c][a] += 1;
        imos[c][b + 1] -= 1;
        imos[d + 1][a] -= 1;
        imos[d + 1][b + 1] += 1; 
    }

    for i in 0..=100 {
        for j in 0..=100 {
            imos[i][j + 1] += imos[i][j];
        }
    }

    for j in 0..=100 {
        for i in 0..=100 {
            imos[i + 1][j] += imos[i][j];
        }
    }

    let mut ans = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            if imos[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
