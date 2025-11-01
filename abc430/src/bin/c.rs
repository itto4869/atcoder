use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_need: usize,   // A
        b_lim: usize,    // B
        s: Bytes,
    }

    let mut ans: u64 = 0;

    // 半開区間 [l, ra) / [l, rb) に含まれる個数
    let (mut ra, mut rb) = (0usize, 0usize);
    let (mut ca, mut cb) = (0usize, 0usize);

    for l in 0..n {
        // a を A 個以上にするため ra を伸ばす
        while ra < n && ca < a_need {
            if s[ra] == b'a' { ca += 1; }
            ra += 1;
        }
        // b が B 未満のまま rb を最大まで伸ばす
        while rb < n && cb + ((s[rb] == b'b') as usize) < b_lim {
            if s[rb] == b'b' { cb += 1; }
            rb += 1;
        }

        // A を満たせないなら、この先も不可能なので打ち切り
        if ca < a_need { break; }

        // r の個数 = max(0, rb - ra)
        if rb > ra {
            ans += (rb - ra) as u64;
        }

        // 左端 l を捨てる（半開区間なので l < ra / rb の時だけ減算）
        if l < ra && s[l] == b'a' { ca -= 1; }
        if l < rb && s[l] == b'b' { cb -= 1; }
    }

    println!("{}", ans);
}
