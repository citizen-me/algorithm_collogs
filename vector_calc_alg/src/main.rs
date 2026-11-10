fn main() {
    let u: Vec<i64> = vec![1, 2, 5];
    let v: Vec<i64> = vec![2, 4, -1];

    println!("{:?}", find_dot_product(&u, &v));
    println!("{:?}", find_cross_product(&u, &v));
}

fn find_dot_product(u: &[i64], v: &[i64]) -> i64 {
    u.iter().zip(v.iter()).map(|(u, v)| u * v).sum()
}

fn find_cross_product(u: &[i64], v: &[i64]) -> Vec<i64> {
    if u.len() != 3 || v.len() != 3 {
        panic!("Cross product is only defined for 3-dimensional vectors.");
    }

    vec![
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}
