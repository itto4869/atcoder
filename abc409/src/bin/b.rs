use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort();
    a.reverse();
    let mut cnt = 0;
    for i in 0..n {
        cnt += 1;
        if cnt >= a[i] {
            println!("{}", a[i].max(cnt - 1));
            return;
        }
    }

    println!("{}", cnt);
}
