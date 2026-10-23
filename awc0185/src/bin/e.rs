use std::collections::HashMap;

use ac_library::{Mod1000000007, StaticModInt};
use cp_library::math::prime_factorization::prime_factorize;
use proconio::{fastout, input};

type Mod107 = StaticModInt<Mod1000000007>;
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let mut map = HashMap::new();
    for &ai in &a {
        let p = prime_factorize(ai as u64);
        for (prime, exp) in p {
            if let Some(&v) = map.get(&prime) {
                map.insert(prime, exp.max(v));
            } else {
                map.insert(prime, exp);
            }
        }
    }

    let mut lcm = Mod107::new(1);
    for (prime, exp) in map {
        lcm = lcm * Mod107::new(prime).pow(exp as u64);
    }

    lcm /= Mod107::new(a[0]);

    println!("{}", lcm);
}
