use hell::matrix::Matrix;

fn main() {
    let a = Matrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let b = Matrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let c = a.add(&b).unwrap();
    let d = a.multiply(&b).unwrap();
    let e = a.transpose();

    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("A + B = {:?}", c);
    println!("A * B = {:?}", d);
    println!("Transpose of A = {:?}", e);
}
