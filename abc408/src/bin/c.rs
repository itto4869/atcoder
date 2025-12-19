use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut imos = vec![0; n + 1];
    for _ in 0..m {
        input! {
            l: Usize1,
            r: Usize1,
        }
        imos[l] += 1;
        imos[r + 1] -= 1;
    }

    for i in 1..=n {
        imos[i] += imos[i - 1];
    }

    imos.remove(n);

    imos.sort();
    println!("{}", imos[0]);
}
