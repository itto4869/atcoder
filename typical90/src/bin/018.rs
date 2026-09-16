use std::f64::consts::PI;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
    }
    for _ in 0..q {
        input! {
            e: f64,
        }
        let e = e % t;
        let rad = 2.0 * PI * (e / t);

        let yt = -rad.sin() * (l / 2.0);
        let zt = (rad.cos() * (l / 2.0) - (l / 2.0)).abs();

        let (dx, dy, dz) = (x, (y - yt).abs(), zt);
        let r = (dx * dx + dy * dy).sqrt();

        let theta = dz.atan2(r) * (180.0 / PI);

        println!("{:.12}", theta);
    }
}
