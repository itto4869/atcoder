use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: u64,
    }
    let mut attack = Vec::with_capacity(2 * n);
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        attack.push((a, 'a'));
        attack.push((b, 'b'));
    }
    attack.sort_by(|a, b| a.0.cmp(&b.0));
    attack.reverse();

    let mut damage = 0;
    let mut cnt = 0;
    for &(d, t) in &attack {
        if t == 'b' {
            damage += d;
            cnt += 1;
        } else {
            while damage < h {
                damage += d;
                cnt += 1;
            }
            break;
        }
        if damage >= h {
            break;
        }
    }
    println!("{}", cnt);
}
