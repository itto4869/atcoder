use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        x: [usize; n],
    }
    let mut ans = Vec::new();
    for i in 0..n {
        let mut ok = true;
        for j in 0..n {
            if i == j {
                continue;
            }

            let dist = x[i].abs_diff(x[j]);
            if dist < d {
                ok = false;
                break;
            }
        }

        if ok {
            ans.push(i + 1);
        }
    }

    
    println!("{}\n{}", ans.len(), ans.iter().format(" "));
}
