use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut abc: [usize; 3],
    }
    abc.sort_unstable();
    let mut ans = usize::MAX;
    for i in 0..10000 {
        for j in 0..10000 {
            let cnt = abc[2] * i + abc[1] * j;
            if cnt > n {
                break;
            }

            let res = n - cnt;
            if (res % abc[0]) == 0 {
                ans = ans.min(i + j + res / abc[0]);
            }
        }
    }

    println!("{}", ans);
}
