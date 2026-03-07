use proconio::{fastout, input};

const MOD: usize = 10007;
const LOG: usize = 31;
#[fastout]
fn main() {
    input! {
        k: usize,
        m: usize,
        pairs: [(usize, u64); k],
    }
    
    let mut pow10 = vec![0usize; LOG];
    pow10[0] = 10 % MOD;
    for p in 1..LOG {
        pow10[p] = pow10[p - 1] * pow10[p - 1] % MOD;
    }

    let size = 10 * LOG * m;
    let mut nxt = vec![0usize; size];
    let mut add = vec![0usize; size];

    let idx = |d: usize, p: usize, r: usize| -> usize { (d * LOG + p) * m + r };

    for d in 0..10 {
        for r in 0..m {
            let x = 10 * r + d;
            let id = idx(d, 0, r);
            nxt[id] = x % m;
            add[id] = x / m;
        }
    }

    for d in 0..10 {
        for p in 0..(LOG - 1) {
            for r in 0..m {
                let id1 = idx(d, p, r);
                let mid = nxt[id1];
                let id2 = idx(d, p, mid);
                let idn = idx(d, p + 1, r);

                nxt[idn] = nxt[id2];
                add[idn] = (add[id1] * pow10[p] + add[id2]) % MOD;
            }
        }
    }

    let mut ans = 0usize;
    let mut rem = 0usize;

    for &(d, mut len) in &pairs {
        let mut p = 0usize;
        while len > 0 {
            if (len & 1) == 1 {
                let id = idx(d, p, rem);
                ans = (ans * pow10[p] + add[id]) % MOD;
                rem = nxt[id];
            }
            len >>= 1;
            p += 1;
        }
    }

    println!("{}", ans);
}
