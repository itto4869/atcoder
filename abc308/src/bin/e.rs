use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Bytes,
    }
    let mut cnt_x = [0i64; 3];
    let mut cnt_m = [0i64; 3];

    for i in 0..n {
        if s[i] == b'X' {
            cnt_x[a[i]] += 1;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        match s[i] {
            b'M' => {
                cnt_m[a[i]] += 1;
            }
            b'E' => {
                for x in 0..3 {
                    for y in 0..3 {
                        ans += cnt_m[x] * cnt_x[y] * mex(x, a[i], y) as i64;
                    }
                }
            }
            b'X' => {
                cnt_x[a[i]] -= 1;
            }
            _ => {}
        }
    }

    println!("{}", ans);
}

fn mex(a: usize, b: usize, c: usize) -> usize {
    let mut used = [false; 4];
    used[a] = true;
    used[b] = true;
    used[c] = true;
    for i in 0..=3 {
        if !used[i] {
            return i;
        }
    }
    4
}