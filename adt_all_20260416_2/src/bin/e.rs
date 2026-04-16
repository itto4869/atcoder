use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u64; n],
    }

    if p[0] >= p[1] {
        println!("0");
        return;
    }

    let mut l_cnt_u = vec![0; n];
    let mut l_cnt_d = vec![0; n];

    for i in 2..n {
        if p[i - 2] < p[i - 1] && p[i - 1] > p[i] {
            l_cnt_u[i] += l_cnt_u[i - 1] + 1;
        }

        if p[i - 2] > p[i - 1] && p[i - 1] < p[i] {
            l_cnt_d[i] += l_cnt_d[i - 1] + 1;
        }
    }

    let mut u_idx = Vec::new();
    let mut d_idx = Vec::new();

    for i in 1..n {
        if l_cnt_u[i] > l_cnt_u[i - 1] {
            u_idx.push(i - 2);
        }

        if l_cnt_d[i] > l_cnt_d[i - 1] {
            d_idx.push(i - 2);
        }
    }

    let mut pre_u_idx = 0;
    let mut pre_d_idx = 0;
    
    let length = u_idx.len().min(d_idx.len());

    for i in 0..length {
        let u_i = u_idx[i];
        let d_i = d_idx[i];

        pre_u_idx = u_i;
        pre_d_idx = d_i;
    }
}
