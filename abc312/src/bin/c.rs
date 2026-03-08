use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m]
    }
    a.sort_unstable();
    b.sort_unstable();
    b.reverse();

    let mut l = 0;
    let mut r = 10u64.pow(9) + 1;


    while r - l > 1 {
        let mid = (r + l) / 2;
        let i = a.partition_point(|&x| x <= mid);
        let j = b.partition_point(|&x| x >= mid);

        if i < j {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", r);
}
