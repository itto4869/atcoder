use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
        u: Bytes,
    }
    for i in 0..(s.len() - u.len() + 1) {
        let mut ok = true;
        for j in 0..u.len() {
            if s[i + j] == b'?' {
                continue;
            } else if s[i + j] == u[j] {
                continue;
            } else {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        } 
    }

    println!("No");
}
