use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut a: [i128; n],
        }
        let k = a[0];
        let mut ok = true;
        if a.iter().all(|&x| x == k) {
            println!("Yes");
            continue;
        }

        if a.iter().all(|x| x.abs() == k.abs()) {
            let mut p_cnt = 0;
            let mut n_cnt = 0;
            for i in 0..n {
                if a[i] == k {
                    p_cnt += 1;
                } else if a[i] == -k {
                    n_cnt += 1;
                }
            }
            if p_cnt + n_cnt == n && p_cnt.min(n_cnt) == n / 2 {
                println!("Yes");
                continue;
            }
        }
        
        a.sort_by(|a, b| a.abs().cmp(&b.abs()));
        for i in 0..(n - 2) {
            if a[i] * a[i + 2] != a[i + 1] * a[i + 1] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
