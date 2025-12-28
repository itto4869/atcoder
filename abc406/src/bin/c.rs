use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u64; n],
    }
    let mut rle = Vec::with_capacity(n);
    let mut curr = 0u64;
    let mut cnt = 0u64;
    for i in 1..n {
        if p[i - 1] < p[i] && curr != 2 {
            cnt += 1;
            curr = 1;
        } else if p[i - 1] < p[i] && curr == 2 {
            rle.push((curr, cnt));
            cnt = 1;
            curr = 1;
        } else if p[i - 1] > p[i] && curr != 1 {
            cnt += 1;
            curr = 2;
        } else if p[i - 1] > p[i] && curr == 1 {
            rle.push((curr, cnt));
            cnt = 1;
            curr = 2;
        }
    }

    rle.push((curr, cnt));

    let mut ans = 0;
    for i in 1..(rle.len() - 1) {
        if rle[i - 1].0 == 1 && rle[i].0 == 2 && rle[i + 1].0 == 1 {
            ans += rle[i - 1].1 * rle[i + 1].1;
        }
    }

    println!("{}", ans);
}
