use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ok = 0;
    let mut ng = l + 1;
    let mut v = Vec::new();
    v.push(a[0]);
    for i in 1..n {
        v.push(a[i] - a[i - 1]);
    }

    v.push(l - a[n - 1]);
    while (ng - ok) > 1 {
        let mid = (ng + ok) / 2;

        let mut x = 0;
        let mut cnt = 0;
        for &vi in &v {
            x += vi;
            if x >= mid {
                cnt += 1;
                x = 0;
            }
        }
        
        if cnt >= k + 1 {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
