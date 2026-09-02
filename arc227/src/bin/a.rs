use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: Chars,
        b: Chars,
        c: Chars,
    }
    let mut a_idxs = Vec::new();
    let mut b_idxs = Vec::new();
    let mut c_idxs = Vec::new();
    for i in 0..(2 * n) {
        if a[i] == '1' {
            a_idxs.push(i);
        }

        if b[i] == '1' {
            b_idxs.push(i);
        }

        if c[i] == '1' {
            c_idxs.push(i);
        }
    }

    let mut v = Vec::with_capacity(n);
    let mut ans_cnt = 0;
    for i in 0..n {
        let (a_idx, b_idx, c_idx) = (a_idxs[i], b_idxs[i], c_idxs[i]);

        let mut idxs = [a_idx, b_idx, c_idx];
        idxs.sort();
        let mid = idxs[1];
        v.push(mid);
        ans_cnt += a_idx.abs_diff(mid) + b_idx.abs_diff(mid) + c_idx.abs_diff(mid);
    }

    let mut idx = 0;
    let mut ans = String::new();
    for &k in &v {
        for _ in idx..k {
            ans.push('0');
        }

        idx = k + 1;
        ans.push('1');
    }

    for _ in (*v.last().unwrap() + 1)..(2 * n) {
        ans.push('0');
    }

    println!("{}\n{}", ans_cnt, ans);
}
