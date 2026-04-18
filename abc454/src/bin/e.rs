use proconio::{fastout, input};

fn push_repeat(ans: &mut String, ch: char, k: usize) {
    for _ in 0..k {
        ans.push(ch);
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a1: usize,
            b1: usize,
        }

        // 必要十分条件
        if n % 2 == 1 || (a1 + b1) % 2 == 0 {
            println!("No");
            continue;
        }

        let a = a1 - 1;
        let b = b1 - 1;

        // 禁止マスを含む 2 行帯の上側
        let s = (a / 2) * 2;
        // 禁止マスを含む 2 列ブロックの左側
        let p = (b / 2) * 2;

        let mut ans = String::new();

        // 1) 上側の 2 行帯
        for _row in (0..s).step_by(2) {
            push_repeat(&mut ans, 'R', n - 1);
            ans.push('D');
            push_repeat(&mut ans, 'L', n - 1);
            ans.push('D');
        }

        // 2) 禁止マスを含む 2 行帯
        // 左側の 2 列ブロック
        for _col in (0..p).step_by(2) {
            ans.push('D');
            ans.push('R');
            ans.push('U');
            ans.push('R');
        }

        // 禁止マスを含む 2 列ブロック
        if a == s {
            // 禁止マスが上側
            ans.push('D');
            ans.push('R');
        } else {
            // 禁止マスが下側
            ans.push('R');
            ans.push('D');
        }

        // 右側の 2 列ブロック
        for _col in (p + 2..n).step_by(2) {
            ans.push('R');
            ans.push('U');
            ans.push('R');
            ans.push('D');
        }

        // 3) 下側の 2 行帯
        for _row in (s + 2..n).step_by(2) {
            ans.push('D');
            push_repeat(&mut ans, 'L', n - 1);
            ans.push('D');
            push_repeat(&mut ans, 'R', n - 1);
        }

        println!("Yes");
        println!("{}", ans);
    }
}