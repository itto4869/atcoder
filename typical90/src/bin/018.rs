use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }
    let r = l / 2.0;
    for &ei in &e {
        let theta = 2.0 * PI * ei / t;

        let yi = -r * theta.sin();
        let zi = r - r * theta.cos();

        let dx = x;
        let dy = yi - y;
        let dist = (dx * dx + dy * dy).sqrt();

        let rad = zi.atan2(dist);
        let deg = rad.to_degrees();

        println!("{}", deg);
    }
}
