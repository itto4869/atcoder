use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort_unstable();
    b.reverse();

    let mut ans = 0;
    for ai in a {
        let bidx = b.partition_point(|&x| x > (ai + d));
        if bidx == m {
            continue;
        }

        let bi = b[bidx];
        
        if ai.abs_diff(bi) > d {
            continue;
        } else {
            ans = ans.max(ai + bi);
        }
    }

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
