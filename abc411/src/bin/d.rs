use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        q: usize,
    }
    let mut op = vec![0; q];
    let mut p = vec![0; q];
    let mut s: Vec<String> = vec!["".to_string(); q];
    for i in 0..q {
        input! {
            c: u64,
        }
        op[i] = c;
        if c == 1 || c == 3{
            input! {
                pi: usize,
            }
            p[i] = pi;
        } else {
            input! {
                pi: usize,
                si: String,
            }
            p[i] = pi;
            s[i] = si;
        }
    }
    let mut target = 0;
    let mut ans: Vec<char> = Vec::with_capacity(1e6 as usize);
    for i in (0..q).rev() {
        if op[i] == 1 {
            if target == p[i] {
                target = 0;
            }
        } else if op[i] == 2 {
            if target == p[i] {
                let mut si: Vec<char> = s[i].chars().rev().collect();
                ans.append(&mut si);
            }
        } else {
            if target == 0 {
                target = p[i];
            }
        }
    }
    ans.reverse();
    let ans: String = ans.iter().collect();
    println!("{}", ans);
}
