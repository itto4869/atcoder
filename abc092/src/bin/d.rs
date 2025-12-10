use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let mut rest_a = a - 1;
    let mut rest_b = b - 1;
    let mut grid = String::with_capacity(10 * 100);

    let half_h = 50;
    let w = 100;
    for i in 0..half_h {
        for j in 0..w {
            if rest_a > 0 && i % 2 == 0 && j % 2 == 0 {
                grid.push('.');
                rest_a -= 1;
            } else {
                grid.push('#');
            }
        }
        grid.push('\n');
    }

    for i in 0..half_h {
        for j in 0..w {
            if rest_b > 0 && i % 2 == 1 && j % 2 == 0 {
                grid.push('#');
                rest_b -= 1;
            } else {
                grid.push('.');
            }
        }
        grid.push('\n');
    }
    println!("{} {}", half_h * 2, w);
    println!("{}", grid);
}
