use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut arr = vec![Vec::new(); n + 1];
    let mut cnt = vec![0; m + 1];
    for i in 0..m {
        input! {
            k: usize,
        }
        for _ in 0..k {
            input! {
                a: usize,
            }
            arr[a].push(i);
            cnt[i] += 1;
        }
    }

    let mut ans = 0;
    for _ in 0..n {
        input! {
            b: usize,
        }
        let cuisines = &arr[b];
        for &cuisine in cuisines {
            cnt[cuisine] -= 1;
            if cnt[cuisine] == 0 {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}
