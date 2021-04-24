#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n],
    }
    // 1 <= x <= 1000 が徐々に狭まるイメージ?
    // x_min, x_maxで考える

    let mut x_min = 1;
    let mut x_max = 1000;

    for i in 0..n {
        if x_min <= a[i] {
            x_min = a[i];
        }
        if b[i] <= x_max {
            x_max = b[i];
        }
    }
    if x_min > x_max {
        println!("0");
        return;
    }

    println!("{}", x_max - x_min + 1);
}
