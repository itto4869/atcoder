use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes
    }
    let mut alphabets = BTreeSet::new();
    for c in s {
        alphabets.insert(c);
    }
    if alphabets.len() == 26 {
        println!("None");
        return;
    }
    
    for (i, c) in alphabets.iter().enumerate() {
        if c - b'a' != i as u8 {
            println!("{}", (b'a' + i as u8) as char);
            return;
        }
    }

    println!("{}", (b'a' + alphabets.len() as u8) as char);
}
