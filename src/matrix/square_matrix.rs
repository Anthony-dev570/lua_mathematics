use crate::scalar::Scalar;

pub trait SquareMatrix<S: Scalar> {
    ///Identifies the identity matrix, signified by a square matrix (NxN) with 1's going diagonally through the matrix.
    const IDENTITY: Self;

    fn identity(value: Option<S>) -> Self;
}