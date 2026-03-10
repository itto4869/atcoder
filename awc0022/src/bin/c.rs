use cp_library::utils::{yes_no, yes_no_custom};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: usize,
    }
    let mut tile_sum = vec![0; n];
    for _ in 0..m {
        input! {
            b: Usize1,
        }
        tile_sum[b] = 1;
    }

    for i in 1..n {
        tile_sum[i] += tile_sum[i - 1];
    }

    for _ in 0..k {
        input! {
            l: Usize1,
            r: Usize1,
        }
        let cnt = if l > 0 {
            tile_sum[r] - tile_sum[l - 1]
        } else {
            tile_sum[r]
        };

        yes_no_custom(cnt >= t, "YES", "NO");
    }
}
