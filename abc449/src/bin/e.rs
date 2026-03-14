use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
        q: usize,
    }
    let mut count: Vec<(i64, usize)> = Vec::new();
    for i in 0..m {
        count.push((0, i));
    }

    for &ai in &a {
        count[ai] = (count[ai].0 + 1, ai);
    }

    count.sort_unstable();
    let mut count_sum = vec![0u64; m];
    count_sum[0] = 1;
    for i in 1..m {
        count_sum[i] = count_sum[i - 1] + ((count[i].0 - count[i - 1].0 + 1) as usize * i) as u64 + 1;
    }
    for _ in 0..q {
        input! {
            x: u64,
        }
        let idx = count_sum.partition_point(|&p| p < x);
        if idx == m {
            let rem = x - count_sum[idx - 1] - 1;
            let idx = rem as usize % m;
            println!("{}", count[idx].1);
        } else {
            let rem = x - count_sum[idx - 1];
            let idx = rem as usize % idx;
            println!("{}", count[idx].1);
        }
    }
}
