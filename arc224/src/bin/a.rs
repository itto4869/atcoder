use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            k: usize,
        }
        for q in 1..=100 {
            let mut x = q * k;
            let mut flag = 0;
            let mut ok = false;
            while x > 0 {
                let r = x % 10;
                x = x / 10;
                if r == 0 && flag == 1 {
                    ok = true;
                    break;
                } else if r == 0 && flag == 0 {
                    flag = 1;
                } else {
                    flag = 0;
                }
            }

            if ok {
                println!("{}", q * k);
                break;
            }
        }
    }
}
