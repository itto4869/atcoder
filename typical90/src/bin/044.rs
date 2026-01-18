use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
    }
    let mut offset = 0;
    for _ in 0..q {
        input! {
            t: usize,
            mut x: usize,
            mut y: usize,
        }

        x = x.saturating_sub(1);
        y = y.saturating_sub(1);
        let mut x_idx = (x as isize - offset as isize) % n as isize;
        if x_idx.is_negative() {
            x_idx += n as isize;
        }

        let mut y_idx = (y as isize - offset as isize) % n as isize;
        if y_idx.is_negative() {
            y_idx += n as isize;
        }

        let x_idx = x_idx as usize;
        let y_idx = y_idx as usize;

        if t == 1 {
            a.swap(x_idx, y_idx);
        } else if t == 2 {
            offset = (offset + 1) % n;
        } else {
            println!("{}", a[x_idx]);
        }
    }
}
