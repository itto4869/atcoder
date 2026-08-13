use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut t_cnt = 0;
    let mut a_cnt = 0;
    for c in s {
        if c == 'T' {
            t_cnt += 1;
        } else {
            a_cnt += 1;
        }

        if t_cnt == ((n + 1) / 2) {
            println!("T");
            break;
        } else if a_cnt == ((n + 1) / 2) {
            println!("A");
            break;
        }
    }
}
