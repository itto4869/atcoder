use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            s: Chars,
        }
        let mut stack = Vec::with_capacity(10usize.pow(6));
        let mut flag = 0;
        for i in 0..s.len() {
            if s[i] == 'A' {
                stack.push(s[i]);
                flag = 1;
            } else if s[i] == 'B' {
                if flag == 1 {
                    if i < (s.len() - 1) && s[i + 1] == 'C' {
                        stack.push(s[i]);
                        flag = 2;
                    } else if i < (s.len() - 1) && s[i + 1] == 'A' {
                        stack.push(s[i]);
                    } else {
                        stack.pop();
                        
                        let n = stack.len();
                        if stack.is_empty() {
                            flag = 0;
                        } else {
                            let c1 = stack[n - 1];
                        if c1 == 'A' {
                            flag = 1;
                        }
                    }
                    }
                } else {
                    stack.push(s[i]);
                }
            } else if s[i] == 'C' {
                if flag == 2 {
                    stack.pop();
                    stack.pop();

                    let n = stack.len();
                    if stack.is_empty() {
                        flag = 0;
                    } else if stack.len() == 1 && stack[n - 1] == 'A' {
                        flag = 1;
                    } else {
let (c1, c2) = (stack[n - 1], stack[n - 2]);
                    if c1 == 'B' && c2 == 'A' {
                        flag = 2;
                    } else if c2 == 'A' {
                        flag = 1;
                    }
                    }

                }
            }
        }

        let mut ans = 0usize;
        for c in stack {
            if c != 'A' {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}
