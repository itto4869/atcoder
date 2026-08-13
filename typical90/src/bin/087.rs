use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: i64,
        k: i64,
        a: [[i64; n]; n],
    }
    
    let mut dist = vec![vec![0; n]; n];
    floyd_warshall(n, p + 1, &mut dist, &a);
    let mut cnt = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if dist[i][j] <= p {
                cnt += 1;
            }
        }
    }

    if cnt == k {
        println!("Infinity");
        return;
    }

    let mut ng = 0;
    let mut ok = p + 1;
    
    while (ok - ng) > 1 {
        let mid = (ng + ok) / 2;
        floyd_warshall(n, mid, &mut dist, &a);

        let mut cnt = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if dist[i][j] <= p {
                    cnt += 1;
                }
            }
        }

        if cnt <= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let l = ok;

    let mut ng = 0;
    let mut ok = p + 1;

    while (ok - ng) > 1 {
        let mid = (ng + ok) / 2;
        floyd_warshall(n, mid, &mut dist, &a);

        let mut cnt = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if dist[i][j] <= p {
                    cnt += 1;
                }
            }
        }

        if cnt < k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let r = ok;
    println!("{}", r - l);
}

fn floyd_warshall(n: usize, x: i64, dist: &mut Vec<Vec<i64>>, graph: &Vec<Vec<i64>>) {
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == -1 {
                dist[i][j] = x;
            } else {
                dist[i][j] = graph[i][j];
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
}