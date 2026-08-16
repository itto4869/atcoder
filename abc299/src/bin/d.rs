use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ok = n;
    let mut ng = 0;
    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        println!("? {}", mid);

        input! {
            s: usize,
        }

        if s == 0 {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    println!("! {}", ng);
}
