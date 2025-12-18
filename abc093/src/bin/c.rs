use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [u64; 3],
    }
    let mut ans = 0;
    abc.sort();
    let mut a = abc[0];
    let mut b = abc[1];
    let c = abc[2];
    loop {
        if c >= b + 2 {
            b += 2;
            ans += 1;
        } else if b >= a + 2 {
            a += 2;
            ans += 1;
        } else {
            break;
        }
    }

    if (c - b) == 0 && (b - a) == 0 {
    } else if (c - b) == 1 && (b - a) == 0 {
        ans += 1;
    } else if (c - b) == 0 && (b - a) == 1 {
        ans += 2;
    } else {
        ans += 3;
    }

    println!("{}", ans);
}
