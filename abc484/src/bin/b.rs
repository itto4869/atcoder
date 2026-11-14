use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        a: [usize; k],
    }
    let &max_a = a.iter().max().unwrap();
    let &min_a = a.iter().min().unwrap();
    if max_a == 9 {
        println!("{}", min_a);
    } else {
        println!("-1");
    }
}
