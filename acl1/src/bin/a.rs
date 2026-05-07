use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }
    let mut y_at_x = vec![0usize; n + 1];
    let mut id_at_x = vec![0usize; n + 1];
    for (i, &(x, y)) in xy.iter().enumerate() {
        y_at_x[x] = y;
        id_at_x[x] = i;
    }

    let inf = n + 1;
    let mut pref_min = vec![inf; n + 1];
    for i in 1..=n {
        pref_min[i] = pref_min[i - 1].min(y_at_x[i]);
    }

    let mut suff_max = vec![0usize; n + 2];
    for i in (1..=n).rev() {
        suff_max[i] = suff_max[i + 1].max(y_at_x[i]);
    }

    let mut ans = vec![0usize; n];

    let mut l = 1;

    for t in 1..n {
        if pref_min[t] > suff_max[t + 1] {
            let size = t - l + 1;
            for x in l..=t {
                let city_id = id_at_x[x];
                ans[city_id] = size;
            }
            l = t + 1;
        }
    }

    let size = n - l + 1;
    for x in l..=n {
        let city_id = id_at_x[x];
        ans[city_id] = size;
    }

    for v in ans {
        println!("{}", v);
    }
}
