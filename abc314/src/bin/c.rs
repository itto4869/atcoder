use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [Usize1; n],
    }
    let mut c_map = Vec::new();
    let mut map = vec![Vec::new(); m];
    let mut idxs = vec![0; m];
    for i in 0..n {
        let idx = c[i];
        map[idx].push(i);
        c_map.push(idx);
    }

    for i in 0..m {
        let sub = &mut map[i];
        if sub.is_empty() {
            continue;
        }

        idxs[i] = sub.len() - 1;
    }

    let mut ans = String::new();
    for i in 0..n {
        let idx = c_map[i];
        let j = idxs[idx];
        let k = map[idx][j];

        idxs[idx] = (idxs[idx] + 1) % map[idx].len();

        ans.push(s[k]);
    }

    println!("{}", ans);
}
