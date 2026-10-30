use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut xy = Vec::with_capacity(n);
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut y_min = i64::MAX;
    let mut y_max = i64::MIN;
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        let nx = x - y;
        let ny = x + y;
        x_min = x_min.min(nx);
        x_max = x_max.max(nx);
        y_min = y_min.min(ny);
        y_max = y_max.max(ny);
        xy.push((x - y, x + y));
    }

    for _ in 0..q {
        input! {
            q: Usize1,
        }
        let (x, y) = xy[q];
        let res = x.abs_diff(x_min).max(x.abs_diff(x_max)).max(y.abs_diff(y_min)).max(y.abs_diff(y_max));

        println!("{}", res);
    }
}
