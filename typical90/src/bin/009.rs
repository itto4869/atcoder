use std::f64::consts::PI;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    let mut ans: f64 = 0.0;

    for j in 0..n {
        let mut angles = Vec::with_capacity(n - 1);

        for i in 0..n {
            if i == j {
                continue;
            }

            let dx = xy[i].0 - xy[j].0;
            let dy = xy[i].1 - xy[j].1;

            let mut theta = dy.atan2(dx) * 180.0 / PI;

            if theta < 0.0 {
                theta += 360.0;
            }

            angles.push(theta);
        }

        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let m = angles.len();

        for &theta in &angles {
            let target = (theta + 180.0) % 360.0;

            let pos = angles.partition_point(|&x| x < target);

            let idx1 = pos % m;

            let idx2 = (pos + m - 1) % m;

            for idx in [idx1, idx2] {
                let phi = angles[idx];

                let d = (theta - phi).abs();
                let angle = d.min(360.0 - d);

                ans = ans.max(angle);
            }
        }
    }

    println!("{:.15}", ans);
}
