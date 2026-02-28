use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

const MOD: u64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut dsu = Dsu::new(n);
    
    let mut idx = m;
    let mut connected = n;
    for i in (0..m).rev() {
        let (u, v) = uv[i];
        if !dsu.same(u, v) {
            connected -= 1;
        }
        dsu.merge(u, v);
        if connected >= 2 {
            idx = i;
        }
    }

    dsu = Dsu::new(n);

    for i in idx..m {
        let (u, v) = uv[i];
        dsu.merge(u, v);
    }

    let mut nums = Vec::new();
    for i in (0..idx).rev() {
        let (u, v) = uv[i];
        if dsu.same(u, v) {
            dsu.merge(u, v);
        } else {
            nums.push(i + 1);
        }
    }

    let &max_exp = nums.iter().max().unwrap();
    let mut two_exp = vec![0; max_exp];
    two_exp[0] = 2;
    for i in 1..max_exp {
        two_exp[i] = (two_exp[i - 1] * 2) % MOD;
    }

    let mut ans = 0;
    for i in nums {
        ans = (ans + two_exp[i - 1]) % MOD;
    }

    println!("{}", ans);
}
