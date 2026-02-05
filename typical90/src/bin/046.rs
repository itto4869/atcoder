use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut a_mod = vec![0; 46];
    let mut b_mod = vec![0; 46];
    let mut c_mod = vec![0; 46];

    for i in 0..n {
        a_mod[a[i] % 46] += 1;
        b_mod[b[i] % 46] += 1;
        c_mod[c[i] % 46] += 1;
    }

    let mut ans = 0u64;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += a_mod[i] * b_mod[j] * c_mod[k];
                }
            }
        }
    }

    println!("{}", ans);
}
