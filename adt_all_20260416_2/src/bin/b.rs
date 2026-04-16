use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let mut ok = [false, false, false];
    if a == 1 || b == 1 {
        ok[0] = true;
    }

    if a == 2 || b == 2 {
        ok[1] = true;
    }

    if a == 3 || b == 3 {
        ok[0] = true;
        ok[1] = true;
    }

    if a == 4 || b == 4 {
        ok[2] = true;
    }

    if a == 5 || b == 5 {
        ok[0] = true;
        ok[2] = true;
    }

    if a == 6 || b == 6 {
        ok[1] = true;
        ok[2] = true;
    }

    if a == 7 || b == 7 {
        ok[0] = true;
        ok[1] = true;
        ok[2] = true;
    }

    let mut ans = 0;
    if ok[0] {
        ans += 1;
    }

    if ok[1] {
        ans += 2;
    }

    if ok[2] {
        ans += 4;
    }

    println!("{}", ans);
}
