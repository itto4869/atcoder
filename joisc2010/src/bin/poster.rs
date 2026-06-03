use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = String::new();
    let mut base = 0;

    for i in 0..n {
        if k <= base + 2usize.pow((n - i) as u32) / 2 {
            ans.push_str(&"J".repeat(2usize.pow((n - i) as u32) / 2));
            ans.push_str(&"O".repeat(2usize.pow((n - i) as u32) / 2));
            break;
        } else {
            base += 2usize.pow((n - i) as u32) / 2;
            ans.push_str(&"I".repeat(2usize.pow((n - i) as u32) / 2));
            if i == (n - 1) {
                ans.push('J');
            }
        }
    }

    println!("{}", ans);
}
