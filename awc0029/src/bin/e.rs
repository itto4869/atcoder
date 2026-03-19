use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![0; n]; n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
            w: u64,
        }
        graph[u][v] = w;
    }

    let inf = 1u64 << 60;
    let mut diff = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == 0 {
                diff[i][j] = inf;
            } else {
                diff[i][j] = graph[i][j];
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                diff[i][j] = diff[i][j].min(diff[i][k] + diff[k][j]);
            }
        }
    }

    let mut graph = vec![vec![inf; k + 1]; k + 1];
    input! {
        s: Usize1,
        k: usize,
        t: [Usize1; k],
    }

    for i in 0..=k {
        for j in 0..=k {
            if i == j {
                continue;
            }

            
        }
    }
}
