use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = String::new();
    for _ in 0..n {
        input! {
            s: Chars
        }
        let c = s[0];
        match c {
            'a' => ans.push('2'),
            'b' => ans.push('2'),
            'c' => ans.push('2'),
            'd' => ans.push('3'),
            'e' => ans.push('3'),
            'f' => ans.push('3'),
            'g' => ans.push('4'),
            'h' => ans.push('4'),
            'i' => ans.push('4'),
            'j' => ans.push('5'),
            'k' => ans.push('5'),
            'l' => ans.push('5'),
            'm' => ans.push('6'),
            'n' => ans.push('6'),
            'o' => ans.push('6'),
            'p' => ans.push('7'),
            'q' => ans.push('7'),
            'r' => ans.push('7'),
            's' => ans.push('7'),
            't' => ans.push('8'),
            'u' => ans.push('8'),
            'v' => ans.push('8'),
            'w' => ans.push('9'),
            'x' => ans.push('9'),
            'y' => ans.push('9'),
            'z' => ans.push('9'),
            _ => unreachable!()
        }
    }
    println!("{}", ans);
}
