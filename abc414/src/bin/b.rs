use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = String::with_capacity(110);
    let mut length = 0;
    for _ in 0..n {
        input! {
            c: String,
            l: usize,
        }
        length += l;
        if length > 100 {
            println!("Too Long");
            return;
        }
        let s = c.repeat(l);
        ans.push_str(&s);
    }
    println!("{}", ans);
}
