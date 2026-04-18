use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: Chars,
            b: Chars,
        }
        let mut a_stack = Vec::new();
        let mut b_stack = Vec::new();
        for &ai in &a {
            let n = a_stack.len();
            if ai != ')' {
                a_stack.push(ai);
            } else {
                if n < 3 {
                    a_stack.push(ai);
                } else if a_stack[n - 1] == 'x' && a_stack[n - 2] == 'x' && a_stack[n - 3] == '(' {
                    a_stack.pop();
                    a_stack.pop();
                    a_stack.pop();
                    
                    a_stack.push('x');
                    a_stack.push('x');
                } else {
                    a_stack.push(ai);
                }
            }
        }

        for &bi in &b {
            let n = b_stack.len();
            if bi != ')' {
                b_stack.push(bi);
            } else {
                if n < 3 {
                    b_stack.push(bi);
                } else if b_stack[n - 1] == 'x' && b_stack[n - 2] == 'x' && b_stack[n - 3] == '(' {
                    b_stack.pop();
                    b_stack.pop();
                    b_stack.pop();
                    
                    b_stack.push('x');
                    b_stack.push('x');
                } else {
                    b_stack.push(bi);
                }
            }
        }

        let mut ok = true;
        if a_stack.len() != b_stack.len() {
            ok = false;
        } else {
            for i in 0..a_stack.len() {
                if a_stack[i] != b_stack[i] {
                    ok = false;
                    break;
                }
            }
        }

        yes_no!(ok);
    }
}
