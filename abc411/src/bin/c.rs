use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut color = vec![0; n + 2];
    let mut ans = 0;
    for _ in 0..q {
        input! {
            a: usize,
        }
        if color[a] == 0 {
            if color[a - 1] == 0 && color[a + 1] == 0 {
                ans += 1;
            } else if color[a - 1] == 1 && color[a + 1] == 1 {
                ans -= 1;
            } else {
            }
        } else {
            if color[a - 1] == 0 && color[a + 1] == 0 {
                ans -= 1;
            } else if color[a - 1] == 1 && color[a + 1] == 1 {
                ans += 1;
            } else {
            }
        }

        color[a] = 1 - color[a];
        println!("{}", ans);
    }
}
