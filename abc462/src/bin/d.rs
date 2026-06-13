use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let mut imos = vec![0; 100_0000 + 1];
    for _ in 0..n {
        input! {
            s: Usize1,
            t: Usize1
        }
        imos[s] += 1;
        imos[((t + 1).saturating_sub(d)).max(s)] -= 1;
    }

    for i in 1..imos.len() {
        imos[i] = imos[i] + imos[i - 1];
    }
    let mut ans = 0;
    for &v in &imos {
        let d = v as usize;
        if d == 0 {
            continue;
        }

        ans += (d * (d - 1)) / 2;
    }

    println!("{}", ans);
}
