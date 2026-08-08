use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut ans = usize::MAX;
    for i in 1..=n {
        let mut cnt = 0;
        for &ci in &c {
            if ci != i {
                cnt += 1;
            }
        }

        ans = ans.min(cnt);
    }

    println!("{}", ans);
}
