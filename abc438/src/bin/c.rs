use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut curr = a[0];
    let mut cnt = 0;
    let mut rle = vec![(0, 0); n];
    let mut delete_cnt = 0;
    let mut idx = 0;
    for i in 0..n {
        if curr == a[i] {
            cnt += 1;

            if cnt == 4 {
                delete_cnt += 1;
                if idx > 0 {
                    (curr, cnt) = rle[idx];
                    idx -= 1;
                } else {
                    curr = 0;
                    cnt = 0;
                }

            }
        } else {
            idx += 1;
            rle[idx] = (curr, cnt);
            curr = a[i];
            cnt = 1;
        }
    }

    let ans = n - 4 * delete_cnt;
    println!("{}", ans);
}
