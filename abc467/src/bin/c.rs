use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; n - 1],
    }
    let mut ca: Vec<usize> = a.iter().copied().collect();
    let mut cnt = 0usize;
    for i in 0..(n - 1) {
        if i == 0 {
            continue;
        } else {
            if (ca[i - 1] + ca[i]) % 2 != b[i - 1] {
                ca[i] += 1;
                cnt += 1;
            }
            if (ca[i] + ca[i + 1]) % 2 != b[i] {
                ca[i + 1] += 1;
                cnt += 1;
            }
        }
    }

    let mut ok = true;
    for i in 0..(n - 1) {
        if (ca[i] + ca[i + 1]) % 2 != b[i] {
            ok = false;
        }
    }

    if !ok {
        cnt = usize::MAX;
    }
    let mut ans = cnt;

    let mut ca: Vec<usize> = a.iter().copied().collect();
    cnt = 0;
    for i in 0..(n - 1) {
        if i == 0 {
            ca[i] += 1;
            cnt += 1;
        } else {
            if (ca[i - 1] + ca[i]) % 2 != b[i - 1] {
                ca[i] += 1;
                cnt += 1;
            } 
            if (ca[i] + ca[i + 1]) % 2 != b[i] {
                ca[i + 1] += 1;
                cnt += 1;
            }
        }
    }

    let mut ok = true;
    for i in 0..(n - 1) {
        if (ca[i] + ca[i + 1]) % 2 != b[i] {
            ok = false;
        }
    }

    if !ok {
        cnt = usize::MAX;
    }
    ans = ans.min(cnt);

    println!("{}", ans);
}
