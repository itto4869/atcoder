use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut s_cnt = vec![0usize; 27];
    let mut t_cnt = vec![0usize; 27];

    for c in s {
        if c == '@' {
            s_cnt[26] += 1;
        } else {
            s_cnt[(c as usize - 'a' as usize)] += 1;
        }
    }

    for c in t {
        if c == '@' {
            t_cnt[26] += 1;
        } else {
            t_cnt[(c as usize - 'a' as usize)] += 1;
        }
    }

    let mut s_at_cnt = 0;
    let mut t_at_cnt = 0;
    let mut ok = true;
    for i in 0..26 {
        if [('a' as usize - 'a' as usize), ('t' as usize - 'a' as usize), ('c' as usize - 'a' as usize), ('o' as usize - 'a' as usize), ('d' as usize - 'a' as usize), ('e' as usize - 'a' as usize), ('r' as usize - 'a' as usize)]
        .contains(&i) {
            s_at_cnt += t_cnt[i].saturating_sub(s_cnt[i]);
            t_at_cnt += s_cnt[i].saturating_sub(t_cnt[i]);
        } else {
            if s_cnt[i] != t_cnt[i] {
                ok = false;
                break;
            }
        }
    }

    if s_at_cnt > s_cnt[26] || t_at_cnt > t_cnt[26] {
        ok = false;
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
