use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        let mut l = 0;
        let mut r = n - 1;
        let mut step = 0;
        for i in 0..(n - 1) {
            if step == 0 {
                if s[i] > s[i + 1] {
                    l = i;
                    step = 1;
                }
            } else if step == 1 {
                if s[l] < s[i + 1] {
                    r = i;
                    step = 2;
                    break;
                }
            } 
        }

        if step == 0 {
            let ans: String = s.iter().collect();
            println!("{}", ans);
        } else if step == 1 {
            let s1: String = s[0..l].iter().collect();
            let s2: String = s[(l + 1)..].iter().collect();
            let mut ans = s1 + &s2;
            ans.push(s[l]);
            println!("{}", ans);
        } else {
            let s1: String = s[0..l].iter().collect();
            let s2: String = s[(l + 1)..=r].iter().collect();
            let s3: String = s[(r + 1)..].iter().collect();

            let mut ans = s1 + &s2;
            ans.push(s[l]);
            ans += &s3;
            println!("{}", ans);
        }
    }
}
