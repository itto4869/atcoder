use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
    }
    let mut point = vec![0; l];
    let mut curr = 0;
    point[curr] += 1;
    for _ in 0..(n - 1) {
        input! {
            d: usize,
        }
        curr = (curr + d) % l;
        point[curr] += 1;
    }

    let mut ans = 0u64;
    if l % 3 != 0 {
        println!("0");
        return;
    }

    for i in 0..(l / 3) {
        ans += point[i] * point[i + l / 3] * point[i + 2 * l / 3];
    }

    println!("{}", ans);
}
