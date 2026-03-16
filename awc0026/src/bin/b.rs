use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
    }
    let mut takahashi = 0;
    let mut aoki = 0;
    for _ in 0..n {
        input! {
            a: u64,
        }
        if (a + takahashi) > k {
            aoki += a;
        } else {
            takahashi += a;
        }
    }

    if takahashi > aoki {
        println!("Takahashi");
    } else if takahashi < aoki {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
