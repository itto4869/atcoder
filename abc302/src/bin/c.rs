use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    for perm in s.iter().permutations(n) {
        let mut ok = true;
        for i in 0..(n - 1) {
            let mut cnt = 0usize;
            for j in 0..m {
                if perm[i][j] != perm[i + 1][j] {
                    cnt += 1;
                }
            }

            if cnt != 1 {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
