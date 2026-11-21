use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    for i in 0..n {
        let mut ok = false;
        for j in 0..m {
            if s[i][j] == 'o' {
                ok = true;
            }
        }

        if !ok {
            println!("Faucet {}", i + 1);
            return;
        }
    }

    for j in 0..m {
        let mut ok = false;
        for i in 0..n {
            if s[i][j] == 'o' {
                ok = true;
            }
        }

        if !ok {
            println!("Drain {}", j + 1);
            return;
        }
    }
}
