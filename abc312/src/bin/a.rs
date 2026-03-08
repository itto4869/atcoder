use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let v = [String::from("ACE"), String::from("BDF"), String::from("CEG"), String::from("DFA"), String::from("EGB"), String::from("FAC"), String::from("GBD")];
    yes_no(v.contains(&s));
}
