use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
    }
    let mut c_max = vec![0; n];
    let mut other = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            c: Usize1,
            v: usize,
        }
        if c_max[c] == 0 {
            c_max[c] = v;
        } else if c_max[c] < v {
            let tmp = c_max[c];
            c_max[c] = v;
            other.push(tmp);
        } else {
            other.push(v);
        }
    }

    c_max.sort_unstable();
    c_max.reverse();

    let mut ans = 0;
    for i in 0..n {
        if i < m {
            ans += c_max[i];
        } else {
            other.push(c_max[i]);
        }
    }

    other.sort_unstable();
    other.reverse();

    for i in 0..(k - m) {
        ans += other[i];
    }

    println!("{}", ans);
}
