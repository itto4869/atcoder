use cp_library::data_structure::weighted_dsu::WeightedDsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut weighted_dsu = WeightedDsu::<i64>::new(n);
    for _ in 0..q {
        input! {
            t: usize,
            x: Usize1,
            y: Usize1,
            v: i64,
        }
        if t == 0 {
            let w = if x % 2 == 1 { -v } else { v };
            weighted_dsu.merge(x, y, w);
        } else {
            if let Some(d) = weighted_dsu.diff(x, y) {
                let bx = if x % 2 == 1 { v } else { -v };
                let by = bx + d;
                let ay = if y % 2 == 1 { by } else { -by };
                println!("{}", ay);
            } else {
                println!("Ambiguous");
            }
        }
    }
}
