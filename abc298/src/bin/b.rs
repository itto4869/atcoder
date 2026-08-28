use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    let mut c = vec![vec![0; n]; n];

    for _ in 0..4 {
        for i in 0..n {
            for j in 0..n {
                c[i][j] = a[n - 1 - j][i];
            }
        }

        for i in 0..n {
            for j in 0..n {
                a[i][j] = c[i][j];
            }
        }

        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if (c[i][j] == 1) && (b[i][j] == 0) {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
