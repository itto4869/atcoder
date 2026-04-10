use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [u64; n],
    }
    let mut ans = u64::MAX;
    for i in 0..n {
        for j in 1..=3 {
            let mut stack = Vec::new();
            if i == 0 {
                stack.push((j, 1));
            } else {
                stack.push((c[0], 1));
            }
            for k in 1..n {
                let mut ok = false;
                let curr_c = if i == k {
                    j
                } else {
                    c[k]
                };
                while let Some((prev_c, prev_cnt)) = stack.pop() {
                    if prev_c == curr_c {
                        stack.push((prev_c, prev_cnt + 1));
                        ok = true;
                        break;
                    } else {
                        if prev_cnt < 4 {
                            stack.push((prev_c, prev_cnt));
                            stack.push((curr_c, 1));
                            ok = true;
                            break;
                        }
                    }
                }

                if !ok {
                    stack.push((curr_c, 1));
                }
            }

            if let Some((prev_c, prev_cnt)) = stack.pop() {
                if prev_cnt < 4 {
                    stack.push((prev_c, prev_cnt));
                }
            }

            ans = ans.min(stack.iter().map(|(_, b)| b ).sum::<u64>());
        }
    }

    println!("{}", ans);
}
