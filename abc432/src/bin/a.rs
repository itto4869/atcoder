use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [u64; 3],
    }
    abc.sort();
    abc.reverse();
    for &n in &abc {
        print!("{}", n);
    }
}
