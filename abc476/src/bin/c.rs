use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = Vec::new();
    v.push(a[0]);
    v.push(a[1]);
    v.push(a[2]);
    v.sort_unstable();
    v.reverse();

    println!("{}", v[2]);
    for k in 3..n {
        for i in 0..3 {
            if v[i] < a[k] {
                v.insert(i, a[k]);
                v.remove(3);
                break;
            }
        }

        println!("{}", v[2]);
    }
}
