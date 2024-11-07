mod matrix;

use matrix::*;

fn main() {
    let x: Matrix<u8, 2, 3> = mtrx![
        [1, 2, 3],
        [4, 5, 6]
    ];

    Matrix::prnt(&x);
    Matrix::prnt(&x.t());
}
