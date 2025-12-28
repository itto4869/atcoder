use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut rows = vec![Vec::new(); h];
    let mut cols = vec![Vec::new(); w];

    for i in 0..n {
        input! {
            x: Usize1,
            y: Usize1,
        }
        rows[x].push(i);
        cols[y].push(i);
    }

    let mut is_deleted = vec![false; n];

    let mut row_done = vec![false; h];
    let mut col_done = vec![false; w];

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            c: usize,
        }

        if c == 1 {
            input! {
                x: Usize1,
            }
            if row_done[x] {
                println!("0");
                continue;
            }

            let mut cnt = 0;
            for &id in &rows[x] {
                if !is_deleted[id] {
                    cnt += 1;
                    is_deleted[id] = true;
                }
            }

            row_done[x] = true;
            println!("{}", cnt);
        } else {
            input! {
                y: Usize1,
            }

            if col_done[y] {
                println!("0");
                continue;
            }
            let mut cnt = 0;
            for &id in &cols[y] {
                if !is_deleted[id] {
                    cnt += 1;
                    is_deleted[id] = true;
                }
            }

            col_done[y] = true;
            println!("{}", cnt);
        }
    }
}
