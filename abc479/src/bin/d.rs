use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        l: [usize; n - 1],
        r: [usize; n - 1],
    }
    let mut l_min = vec![1 << 60; n - 1];
    let mut r_min = vec![1 << 60; n - 1];
    l_min[n - 2] = l[n - 2];
    r_min[0] = r[0];

    for i in 1..(n - 1) {
        l_min[n - 2 - i] = l_min[n - 2 - (i - 1)].min(l[n - 2 - i]);
        r_min[i] = r_min[i - 1].min(r[i]);
    }

    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1,
        }
        if s == 0 {
            println!("{}", r[s]);
        } else if s == (n - 1) {
            println!("{}", l[s - 1]);
        } else if s < t {
            let ans = r[s].min(l[s - 1] + r_min[s - 1]);
            println!("{}", ans);
        } else if s > t {
            let ans = l[s - 1].min(r[s] + l_min[s]);
            println!("{}", ans);
        }
    }
}
