use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph[b].push(a);
    }

    let mut v = Vec::new();
    for i in 0..n {
        if graph[i].is_empty() {
            v.push(i);
        }
    }

    if v.len() == 1 {
        println!("{}", v[0] + 1);
    } else {
        println!("-1");
    }
}
