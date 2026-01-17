use ac_library::Dsu;
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
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut connected_components = 0;
    let mut already_connected = vec![false; n];
    let mut to_be_erased = 0;

    let mut dsu = Dsu::new(n);

    for i in 0..n {
        connected_components += 1;
        if already_connected[i] {
            to_be_erased -= 1;
        }

        for &j in &graph[i] {
            if j < i {
                if !dsu.same(i, j) {
                    dsu.merge(i, j);
                    connected_components -= 1;
                }
            } else {
                if !already_connected[j] {
                    to_be_erased += 1;
                }
                already_connected[j] = true;
            }
        }

        if connected_components == 1 {
            println!("{}", to_be_erased)
        } else {
            println!("-1");
        }
    }
}
