use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    }
    let mut ans = 0i64;
    for y in d..=u {
        ans += calc_cnt(l, r);

        if l < -(y.abs()) && y.abs() < r {
            ans -= calc_cnt(-(y.abs()), y.abs());
            ans += if y.abs() % 2 == 0 {
                y.abs() - -(y.abs()) + 1
            } else {
                0
            };
        } else if r <= -(y.abs()) || l >= y.abs() {
            continue;
        } else if l < -(y.abs()) && -(y.abs()) <= r && r <= y.abs() {
            ans -= calc_cnt(-(y.abs()), r);
            ans += if y.abs() % 2 == 0 {
                r - -(y.abs()) + 1
            } else {
                0
            };
        } else if -(y.abs()) <= l && l <= y.abs() && y.abs() < r {
            ans -= calc_cnt(l, y.abs());
            ans += if y.abs() % 2 == 0 {
                y.abs() - l + 1
            } else {
                0
            };
        } else if -(y.abs()) <= l && y.abs() >= r {
            ans -= calc_cnt(l, r);
            ans += if y.abs() % 2 == 0 {
                r - l + 1
            } else {
                0
            };
        }

    }

    println!("{}", ans);
}

fn calc_cnt(a: i64, b: i64) -> i64 {
    if a.abs() % 2 == 1 && b.abs() % 2 == 1 {
        (b - a) / 2
    } else if a.abs() % 2 == 0 && b.abs() % 2 == 0 {
        (b - a) / 2 + 1
    } else {
        (b - a + 1) / 2
    }
}