use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        x: Usize1,
    }
    let v = ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd'];
    let mut ans = String::new();
    for i in 0..v.len() {
        if i == x {
            continue;
        } else {
            ans.push(v[i]);
        }
    }

    println!("{}", ans);
}
