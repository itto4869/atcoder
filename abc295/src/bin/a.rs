use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    let words = ["and", "not", "that", "the", "you"];
    let mut ok = false;
    for wi in w {
        for word in words {
            if wi == word {
                ok = true;
            }
        }
    }

    yes_no!(ok);
}
