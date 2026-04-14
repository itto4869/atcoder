use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut stack = Vec::new();
    let mut ans = String::new();
    for c in s {
        if c == '+' || c == '-' || c == '*' || c == '/' {
            stack.push(c);
        } else if c == '(' {
            ans.push('(');
        } else if c == ')' {
            ans.push(')');
            stack.pop();
        } else if c == ',' {
            ans.push(*(stack.last().unwrap()));
        } else {
            ans.push(c);
        }
    }

    println!("{}", ans);
}
