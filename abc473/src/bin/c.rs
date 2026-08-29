use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut v = vec![0; k];
    for _ in 0..n {
        input! {
            a: Usize1,
        }
        v[a] += 1usize;
    }

    let &max_a = v.iter().max().unwrap();
    let mut ans = 0;
    for va in v {
        if (va == max_a) || (va == max_a - 1) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
