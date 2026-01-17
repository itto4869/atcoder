use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Bytes,
    }
    let mut t = Vec::with_capacity(n);
    let mut cnt_o = 0;
    for i in 0..n {
        if s[i] == b'.' {
            t.push(s[i]);
        } else if s[i] == b'o' {
            t.push(s[i]);
            cnt_o += 1;
        } else if i == 0 && i + 1 < n && s[i + 1] == b'o' {
            t.push(b'.');
        } else if i == (n - 1) && i > 0 && s[i - 1] == b'o' {
            t.push(b'.');
        } else if i > 0 && i < (n - 1) && (s[i - 1] == b'o' || s[i + 1] == b'o') {
            t.push(b'.');
        } else {
            t.push(b'?');
        }
    }

    if k == cnt_o {
        let t: String = t.iter().map(|&c|
            if c == b'?' {
                '.'
            } else {
                c as char
            }).collect();
        println!("{}", t);
        return;
    }

    let mut comp_t = Vec::with_capacity(n);
    let mut c = t[0];
    let mut cnt = 1;
    for i in 1..n {
        if t[i] != c {
            comp_t.push((c, cnt));
            c = t[i];
            cnt = 1;
        } else {
            cnt += 1;
        }
    }

    comp_t.push((c, cnt));

    let mut m = 0;
    for &(c, cnt) in &comp_t {
        if c == b'o' {
            m += cnt;
        } else if c == b'?' {
            m += (cnt + 1) / 2;
        }
    }

    if m == k {
        let mut ans = Vec::with_capacity(n);
        for &(c, cnt) in &comp_t {
            if c != b'?' {
                for _ in 0..cnt {
                    ans.push(c as char);
                }
            } else {
                if cnt % 2 == 1 {
                    for i in 0..cnt {
                        if i % 2 == 0 {
                            ans.push('o');
                        } else {
                            ans.push('.');
                        }
                    }
                } else {
                    for _ in 0..cnt {
                        ans.push('?');
                    }
                }
            }
        }

        let ans: String = ans.iter().collect();
        println!("{}", ans);
        return;
    }

    let ans: String = t.iter().map(|&c| c as char).collect();
    println!("{}", ans);
}
