use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        tx: [(u64, Usize1); n],
    }
    let mut stack = vec![Vec::new(); n];
    let mut imos = vec![0; n];
    let mut v = vec![false; n];
    for (i, &(t, x)) in tx.iter().enumerate() {
        if t == 1 {
            stack[x].push(i);
        } else {
            if let Some(idx) = stack[x].pop() {
                imos[idx] += 1;
                imos[i] -= 1;
                v[idx] = true;
            } else {
                println!("-1");
                return;
            }
        }
    }
    
    for i in 1..n {
        imos[i] += imos[i - 1];
    }

    let k = imos.iter().max().unwrap();
    println!("{}", k);

    let mut ans = Vec::new();
    for i in 0..n {
        let (ti, _) = tx[i];
        if ti == 1 {
            if v[i] {
                ans.push(1);
            } else {
                ans.push(0);
            }
        } else {
            continue;
        }
    }

    println!("{}", ans.iter().format(" "));
}
