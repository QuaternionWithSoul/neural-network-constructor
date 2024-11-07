mod matrix;

use matrix::*;

fn main() {
    let a: Matrix<u8, 1, 2> = mtrx![1, 0];
    let b: Matrix<u8, 1, 2> = mtrx![0, 1];

    let x: Matrix<Matrix<u8, 1, 2>, 1, 2> = mtrx![a, b];

    println!("{:?}", x.into_inner());
}
