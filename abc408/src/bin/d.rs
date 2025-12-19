use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Bytes,
        }

        // そもそも '1' がない場合は答えは 0
        let one_num = s.iter().filter(|&&c| c == b'1').count();
        if one_num == 0 {
            println!("0");
            continue;
        }

        let mut enc = Vec::with_capacity(n);
        let mut c = s[0];
        let mut cnt = 1;
        for i in 1..n {
            if s[i] == c {
                cnt += 1;
            } else {
                enc.push((c, cnt));
                c = s[i];
                cnt = 1;
            }
        }
        enc.push((c, cnt));

        if let Some(&(c, _)) = enc.first() {
            if c == b'0' { 
                enc.remove(0); 
            }
        }

        if !enc.is_empty() {
            if let Some(&(c, _)) = enc.last() {
                if c == b'0' { 
                    enc.pop(); 
                }
            }
        }


        let scores: Vec<i64> = enc.iter().map(|&(c, count)| {
            if c == b'1' {
                count as i64
            } else {
                -(count as i64)
            }
        }).collect();

        let mut max_so_far = 0;
        let mut max_ending_here = 0;
        
        for &x in &scores {
            max_ending_here = max_ending_here + x;

            if max_ending_here < 0 {
                max_ending_here = 0;
            }
            if max_so_far < max_ending_here {
                max_so_far = max_ending_here;
            }
        }

        let ans = one_num as i64 - max_so_far;
        println!("{}", ans);
    }
}