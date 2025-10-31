use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(u64, u64); n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut stu1 = Vec::with_capacity(n);
    let mut stu2 = Vec::with_capacity(n);
    let mut stu1_sum = 0;
    let mut stu2_sum = 0;
    for &(c, p) in cp.iter().rev() {
        match c {
            1 => stu1_sum += p,
            2 => stu2_sum += p,
            _ => unreachable!(),
        }
        stu1.push(stu1_sum);
        stu2.push(stu2_sum);
    }
    stu1.reverse();
    stu2.reverse();
    for &(l, r) in &lr {
        let num1 = if r == n {
            stu1[l - 1]
        } else {
            stu1[l - 1] - stu1[r]
        };
        let num2 = if r == n {
            stu2[l - 1]
        } else {
            stu2[l - 1] - stu2[r]
        };
        println!("{} {}", num1, num2);
    }
}
