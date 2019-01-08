extern crate num;

use num::NumCast;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub enum DType {
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
}

pub trait ToDType {
    fn dtype() -> DType;
}

impl ToDType for i8 {
    fn dtype() -> DType {
        DType::Int8
    }
}

impl ToDType for i16 {
    fn dtype() -> DType {
        DType::Int16
    }
}

impl ToDType for i32 {
    fn dtype() -> DType {
        DType::Int32
    }
}


impl ToDType for i64 {
    fn dtype() -> DType {
        DType::Int64
    }
}

impl ToDType for f32 {
    fn dtype() -> DType {
        DType::Float32
    }
}


impl ToDType for f64 {
    fn dtype() -> DType {
        DType::Float64
    }
}

#[derive(Debug)]
pub struct Matrix2d<T> {
    dtype: DType,
    shape: (usize, usize),
    data: Vec<T>,
}

impl<T: ToDType + NumCast + Clone + Add<Output = T>> Matrix2d<T> {
    pub fn ones(rows: usize, cols: usize) -> Matrix2d<T> {
        let one = T::from(1).unwrap();
        let data = vec![one; rows * cols];
        Matrix2d {
            dtype: T::dtype(),
            shape: (rows, cols),
            data: data,
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix2d<T> {
        let zero = T::from(0).unwrap();
        let data = vec![zero; rows * cols];
        Matrix2d {
            dtype: T::dtype(),
            shape: (rows, cols),
            data: data,
        }
    }

    pub fn new(rows: usize, cols: usize) -> Matrix2d<T> {
        Matrix2d::zeros(rows, cols)
    }

    pub fn add(&self, rhs: &Matrix2d<T>) -> Matrix2d<T> {
        assert_eq!(self.shape, rhs.shape);

        let mut out = Matrix2d::<T>::ones(self.shape.0, self.shape.1);
        for ((zval, l), r) in out.data.iter_mut().zip(&self.data).zip(&rhs.data) {
            *zval = l.clone() + r.clone();
        }
        out
    }
}
