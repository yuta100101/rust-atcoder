use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; 1<<n]
    }
    let mut max1: i32 = 0;
    let mut max2: i32 = 0;
    let mut max_index1: usize = 0;
    let mut max_index2: usize = 0;
    for i in 0..1<<(n-1) {
        if a[i] > max1 {
            max1 = a[i];
            max_index1 = i;
        }
        let index2 = i + (1<<n-1);
        if a[index2] > max2 {
            max2 = a[index2];
            max_index2 = index2;
        }
    }
    if max1 < max2 {
        println!("{}", max_index1 + 1);
    } else {
        println!("{}", max_index2 + 1);
    }
}
