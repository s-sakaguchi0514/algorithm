fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    println!("半角スペース区切りで複数の整数を入力してください");
    let mut vec = read_vec::<i32>();
    let n = vec.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
    println!("{:?}", vec);
}