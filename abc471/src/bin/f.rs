use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut v = Vec::new();
    let mut zero_v = Vec::new();
    let mut u = Vec::new();
    for _ in 0..n {
        input! {
            mut s: Chars,
        }
        let mut x = 0;
        s.reverse();
        for (i, &c) in s.iter().enumerate() {
            x += 10usize.pow(i as u32) * (c as usize - '0' as usize);
        }

        if x == 0 {
            u.push((s.len(), 0));
            zero_v.push(s.len());
        } else {
            u.push((x.ilog10() as usize + 1, x));
            v.push(x);
        }
    }

    v.sort_unstable();
    v.reverse();

    zero_v.sort_unstable();
    zero_v.reverse();

    u.sort_unstable();
    u.reverse();

    if v.is_empty() {
        println!("0");
        return;
    }

    let mut ans = Vec::new();
    let mut back_ans = Vec::new();
    for i in 0..k {
        let (d, ui) = u[i];
        if ui == 0 {
            back_ans.push("0".repeat(d));
        } else {
            ans.push(ui.to_string());
        }
    }
    if ans.is_empty() {
        for i in k..n {
            let (d, ui) = u[i];
            if ui != 0 {
                ans.push(ui.to_string());
            }
        }

        back_ans.sort_unstable();
        back_ans.remove(0);
    }
    ans.sort_unstable();
    ans.reverse();
    let mut ans: String = ans.into_iter().collect();
    let back_ans: String = back_ans.into_iter().collect();
    ans.push_str(&back_ans);
    println!("{}", ans);
}
