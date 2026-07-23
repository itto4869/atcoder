use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xyzw: [(usize, usize, usize, u64); q],
    }

    let constraints: Vec<(usize, u64)> = xyzw
        .into_iter()
        .map(|(x, y, z, w)| {
            let variable_mask = (1usize << (x - 1))
                | (1usize << (y - 1))
                | (1usize << (z - 1));
            
            (variable_mask, w)
        })
        .collect();

    let mut ans = 1u64;

    for bit in 0..60usize {
        let mut ways = 0u64;

        for state in 0usize..(1usize << n) {
            let mut ok = true;

            for &(variable_mask, w) in &constraints {
                let actual = if state & variable_mask != 0 {
                    1u64
                } else {
                    0u64
                };

                let expected = (w >> bit) & 1;

                if actual != expected {
                    ok = false;
                    break;
                }
            }

            if ok {
                ways += 1;
            }
        }

        ans = ans * ways % MOD;
    }

    println!("{}", ans);
}
