use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
        b: [u64; n],
    }
    let mut k = Some(k);
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        if let Some(l) = k {
            k = l.checked_sub(ai.abs_diff(bi));
        } else {
            break;
        }
    }

    if let Some(l) = k {
        if l % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
