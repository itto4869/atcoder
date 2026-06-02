use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        mut x: usize,
        mut y: [usize; m],
    }
    for _ in 0..k {
        if x & 1 == 0 {
            x = (1 << (n - 1)) | (x >> 1);
        } else {
            break;
        }
    }
    
    y.push(x);
    let mut ans = 0;
    for p in 0..(n - 1) {
        let mut a = 0;
        let mut b = 0;

        for i in 0..=p {
            let low_mask = (1usize << (p + 1)) - 1;
            let high_width = n - p - 1;
            let high_mask = (1usize << high_width) - 1;

            for &yi in &y {
                a += yi & low_mask;
                b += (yi >> (p + 1)) & high_mask;
            }
        }

        ans = ans.max(a + b);
    }

    println!("{}", ans);
}
