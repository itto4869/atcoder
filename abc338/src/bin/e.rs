use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![(true, 0); 2 * n];
    for i in 0..n {
        input! {
            a: Usize1,
            b: Usize1,
        }
        let (a, b) = (a.min(b), a.max(b));
        v[a] = (false, i);
        v[b] = (true, i);
    }

    let mut stack = Vec::new();
    for i in 0..(2 * n) {
        let (t, id) = v[i];
        if !t {
            stack.push(id);
        } else {
            if let Some(top) = stack.pop() {
                if top != id {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
