mod matrix2d;

fn main() {
    let x = matrix2d::Matrix2d::<i32>::ones(2, 2);
    let y = matrix2d::Matrix2d::<f32>::zeros(2, 4);
    let z = matrix2d::Matrix2d::<f64>::new(2, 2);

    println!("X + X {:?}", x.add(&x));
    println!("Y {:?}", y);
    println!("Z {:?}", z);

}
