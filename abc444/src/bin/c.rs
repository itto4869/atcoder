use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort_unstable();
    let mut ans = Vec::new();
    if n % 2 == 1 {
        ans.push(a[n - 1]);
    } else {
        let s = a[0] + a[n - 1];
        let mut ok = true;
        for i in 0..(n / 2) {
            if s != (a[i] + a[n - 1 - i]) {
                ok = false;
                break;
            }
        }
        if ok {
            ans.push(a[0] + a[n - 1]);
        }

        let mut ok = true;
        let l = a.iter().max().unwrap();
        let idx = a.partition_point(|x| x < l);
        
        if idx % 2 == 0 && idx > 0 {
            let s = a[n - 1];
            for i in 0..(idx / 2) {
                if s != (a[i] + a[idx - 1 - i]) {
                    ok = false;
                    break;
                }
            }
            
            if ok {
                ans.push(a[n - 1]);
            }
        } else if idx == 0 {
            ans.push(a[n - 1]);
        }
    }

    ans.sort_unstable();
    println!("{}", ans.iter().format(" "));
}
