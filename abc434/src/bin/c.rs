use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize
    }
    for _ in 0..t {
        input! {
            n: usize,
            h: u64,
        }
        let mut c_t = 0;
        let mut bottom = h;
        let mut top = h;
        let mut ok = true;
        for _ in 0..n {
            input! {
                t: u64,
                l: u64,
                u: u64,
            }
            let dt = t - c_t;
            
            let bottom_t = bottom + dt;
            let bottom_b = bottom.saturating_sub(dt);

            let top_t = top + dt;
            let top_b = top.saturating_sub(dt);

            if bottom_b > u {
                ok = false;
            }

            if top_t < l {
                ok = false;
            }

            bottom = l.max(bottom_b);
            top = u.min(top_t);
            c_t = t;

        }

        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
