use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut res = String::new();
    let mut flag = true;
    for (i, &c) in s.iter().enumerate() {
        if c == b'#' {
            res.push_str(&((i + 1).to_string()));
            if flag {
                res.push(',');
                flag = false;
            } else {
                println!("{}", res);
                res.clear();
                flag = true;
            }
        }
    }
}
