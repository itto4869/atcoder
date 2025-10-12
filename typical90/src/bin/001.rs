use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: u64,
        k: u64,
        mut a: [u64; n]
    }
    let mut middle = l / 2;
    let (mut left, mut right) = if is_divide(&a, middle, &n, &k, &l) {
        (middle, l)
    } else {
        (0, middle)
    };

    while left + 1 < right {
        middle = (left + right) / 2;
        (left, right) = if is_divide(&a, middle, &n, &k, &l) {
            (middle, right)
        } else {
            (left, middle)
        };
    }

    if is_divide(&a, right, &n, &k, &l) {
        println!("{}", right);
    } else {
        println!("{}", left);
    }
}

fn is_divide(yokan: &Vec<u64>, m: u64, n: &usize, k: &u64, l: &u64) -> bool {
    let mut length = yokan[0];
    let mut cut = 0;
    if length >= m {
        length = 0;
        cut += 1;
    }
    for i in 1..*n {
        let piece = yokan[i] - yokan[i - 1];
        length += piece;
        if length >= m && cut < *k {
            length = 0;
            cut += 1;
        }
    }
    length += l - yokan[n - 1];
    if cut < *k {
        false
    }
    else if length < m {
        false
    } else {
        true
    }
}