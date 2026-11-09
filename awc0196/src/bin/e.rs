use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut dp_a_p = vec![0usize; 1 << 21];
    let mut dp_a_c = vec![0usize; 1 << 21];
    let mut dp_b_p = vec![0usize; 1 << 21];
    let mut dp_b_c = vec![0usize; 1 << 21];
    dp_a_p[0] = 1;
    dp_b_p[0] = 1;
    for i in 1..=n {
        let ai = a[i - 1];
        let bi = b[i - 1];
        for j in 0..(1 << 21) {
            dp_a_c[j] += dp_a_p[j] + dp_a_p[j ^ ai];
            dp_b_c[j] += dp_b_p[j] + dp_b_p[j ^ bi];
        }

        dp_a_p = dp_a_c;
        dp_b_p = dp_b_c;

        dp_a_c = vec![0usize; 1 << 21];
        dp_b_c = vec![0usize; 1 << 21];
    }

    let mut ans = 0;
    for x in 0..=s {
        let y = s - x;
        ans += dp_a_p[x] * dp_b_p[y];
    }

    println!("{}", ans);
}
