use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut l_cnt = 0;
    let mut stack = Vec::new();
    for i in 0..n {
        let c = s[i];
        if c == '(' {
            l_cnt += 1;
            stack.push(c);
        } else if c == ')' && l_cnt > 0 {
            l_cnt -= 1;
            while let Some(t) = stack.pop() {
                if t == '(' {
                    break;
                }
            }
        } else {
            stack.push(c);
        }
    }

    let ans: String = stack.iter().collect();
    if !ans.is_empty() {
        println!("{}", ans);
    }
}
