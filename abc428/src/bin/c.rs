use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut stack: Vec<(u64, u64, bool)> = Vec::with_capacity(q);
    
    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                c: char,
            }
            let (mut l_cnt, mut r_cnt, mut ok) = if stack.is_empty() {
                (0, 0, true)
            } else {
                *(stack.last().unwrap())
            };

            if c == '(' {
                l_cnt += 1;
                stack.push((l_cnt, r_cnt, ok));
            } else {
                r_cnt += 1;
                if r_cnt > l_cnt {
                    ok = false;
                }
                stack.push((l_cnt, r_cnt, ok));
            }

        } else {
            stack.pop();
        }

        if let Some(&(lc, rc, ok)) = stack.last() {
            yes_no!(ok && (lc == rc));
        } else {
            println!("Yes");
        }
    }
}
