use cp_library::graph::dijkstra;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n :usize,
        q: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut rv = vec![0; n];
    for i in 1..n {
        rv[i] = rv[i - 1] + a[i - 1];
    }

    let mut lv = vec![0; n];
    for i in (0..(n - 1)).rev() {
        lv[i] = lv[i + 1] + a[i];
    }

    let mut graph = vec![Vec::new(); n + 1];
    for i in 0..n {
        if i == (n - 1) {
            graph[0].push((n - 1, a[n - 1]));
            graph[n - 1].push((0, a[n - 1]));
        } else {
            graph[i].push((i + 1, a[i]));
            graph[i + 1].push((i, a[i]));
        }
    }

    for i in 0..n {
        graph[i].push((n, b[i]));
        graph[n].push((i, b[i]));
    }

    let first_d = dijkstra(&graph, 0);
    let end_d = dijkstra(&graph, n);

    for _ in 0..q {
        input! {
            s: Usize1,
            r: Usize1
        }
        if r == n {
            println!("{}", end_d[s]);
        } else if s == 0 {
            println!("{}", first_d[r]);
        } else {
            let common_d = rv[r] - rv[s];
            let l_d = first_d[s] + first_d[r];
            let r_d = end_d[s] + end_d[r];
            println!("{}", common_d.min(l_d).min(r_d));
        }
    }
}
