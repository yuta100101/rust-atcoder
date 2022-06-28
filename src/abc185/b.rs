use proconio::input;

fn main() {
    input! {
        n: i32,
        m: usize,
        t: i32,
        v: [(i32, i32); m],
    }
    let mut vol: i32 = n;
    let mut before: i32 = 0;
    let last: i32 = v[m-1].1;
    let mut is_ok: bool = true;
    for (a, b) in v {
        vol -= a - before;
        if vol <= 0 {
            is_ok = false;
            break
        }
        vol += b - a;
        vol = vol.min(n);
        before = b;
    }
    vol -= t - last;
    if vol <= 0 {
        is_ok = false;
    }
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
