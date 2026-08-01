use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut imos = vec![0usize; n + 1];
    imos[0] = if s[0] == 'o' {
        1
    } else {
        0
    };

    for i in 1..n {
        imos[i] = if s[i] == 'o' {
            imos[i - 1] + 1
        } else {
            imos[i - 1]
        };
    }

    let mut k = 1;
    let mut idx = 0;
    while k <= n && idx < n {
        if s[idx] == 'o' {
            idx += 1;
            continue;
        }
        if imos[idx].saturating_sub((idx + 1).saturating_sub(k)) == 0 {
            println!("{}", idx + 1);
            k += 1;
        }

        idx += 1;
    }

    for _ in k..=n {
        println!("{}", n);
    }
}
