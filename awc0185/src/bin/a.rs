use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [usize; 3],
    }
    let &max_v = abc.iter().max().unwrap();
    for i in 1..=3 {
        if abc[i - 1] == max_v {
            println!("{}", i);
            break;
        }
    }
}
