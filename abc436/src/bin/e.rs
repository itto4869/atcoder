use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }
    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            let mut cycle_len = 0;
            let mut curr = i;

            while !visited[curr] {
                visited[curr] = true;
                curr = p[curr];
                cycle_len += 1;
            }

            ans += (cycle_len * (cycle_len - 1)) / 2;
        }
    }

    println!("{}", ans);
}
