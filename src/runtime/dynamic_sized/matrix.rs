use alloc::{format, vec};
use alloc::vec::Vec;
use core::mem::size_of;
use core::ops::{BitXor, Mul, Not};

use crate::complex::Complex;
use crate::error::{QuantumError, Result};
use crate::runtime::ket::{IndexType, Ket};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DynamicSizedMatrix {
    matrix: Vec<Complex>,
    width: usize,
    qbit_size: usize,
}

impl DynamicSizedMatrix {
    pub(crate) fn new(matrix: Vec<Complex>) -> Result<DynamicSizedMatrix> {
        let len = matrix.len();
        if len <= 1 {
            return Err(QuantumError(
                format!("Expecting matrix length to be larger than 1 but was {}", len)
            ))
        }
        if len == 0 || (len & (len - 1)).not() == 0 {
            return Err(QuantumError(
                format!("Expecting matrix length to be power of two but was {}", len)
            ))
        }
        for i in 0..8 * size_of::<usize>() {
            if (len & (0x1 << i)) > 0 {
                return Ok(DynamicSizedMatrix {
                    matrix,
                    width: 0x1 << (i / 2),
                    qbit_size: i / 2,
                });
            }
        }

        Err(QuantumError(
            format!("Expecting matrix length to be a square root of a power of two but was {}", len)
        ))
    }

    pub(crate) fn apply(&self, ket: Ket, wires: &[usize]) -> Ket {
        // Assert all qbits exist and are different
        assert_eq!(self.qbit_size, wires.len());
        for i in 0..wires.len() {
            assert!(ket.size() >= wires[i]);
            for j in 0..wires.len() {
                if i != j {
                    assert_ne!(wires[i], wires[j]);
                }
            }
        }

        let old_vec = &ket.vec;
        let mut new_ket = Ket::new_from(&ket);
        #[cfg(feature = "rayon")]
            let iter = new_ket.vec.par_iter_mut();
        #[cfg(not(feature = "rayon"))]
            let iter = new_ket.vec.iter_mut();
        iter.enumerate().for_each(|(ket_idx, new_ket_value)| {
            let mut ket_idx: IndexType = ket_idx.into();
            let mut y = IndexType::from(0);
            for pos in 0..self.qbit_size {
                y.set(pos, ket_idx.get(wires[self.qbit_size -1 - pos]).unwrap());
            }
            let y: usize = y.into();

            for x in 0..self.width {
                let x_idx: IndexType = x.into();
                for pos in 0..self.qbit_size {
                    ket_idx.set(wires[self.qbit_size -1 - pos], x_idx.get(pos).unwrap());
                }
                let ket_idx: usize = ket_idx.clone().into();

                let v = *old_vec.get(ket_idx).unwrap();
                let matrix_value = *{ self.matrix.get(y * self.width + x).unwrap() };
                *new_ket_value = *new_ket_value + matrix_value * v;
            }
        });

        new_ket
    }

    pub(crate) fn size(&self) -> usize {
        self.qbit_size
    }
}

impl Mul for DynamicSizedMatrix {
    type Output = DynamicSizedMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix: Vec<Complex> = vec![Complex::zero(); self.matrix.len()];
        for y in 0..self.width {
            for x in 0..self.width {
                for i in 0..self.width {
                    *{ matrix.get_mut(y * self.width + x).unwrap() } =
                        *{ matrix.get(y * self.width + x).unwrap() }
                            + *{ self.matrix.get(y * self.width + i).unwrap() }
                            * *{ rhs.matrix.get(i * self.width + x).unwrap() };
                }
            }
        }

        DynamicSizedMatrix::new(matrix).unwrap()
    }
}

impl<T: Mul<Complex, Output = Complex> + Copy> Mul<T> for DynamicSizedMatrix {
    type Output = DynamicSizedMatrix;

    fn mul(self, rhs: T) -> Self::Output {
        let mut matrix: Vec<Complex> = vec![Complex::zero(); self.matrix.len()];
        for x in 0..self.width {
            for y in 0..self.width {
                *{ matrix.get_mut(x * self.width + y).unwrap() } =
                    rhs.mul(*{ self.matrix.get(x * self.width + y).unwrap() });
            }
        }

        DynamicSizedMatrix::new(matrix).unwrap()
    }
}

impl Mul<DynamicSizedMatrix> for Complex {
    type Output = DynamicSizedMatrix;

    #[inline(always)]
    fn mul(self, rhs: DynamicSizedMatrix) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<DynamicSizedMatrix> for f64 {
    type Output = DynamicSizedMatrix;

    #[inline(always)]
    fn mul(self, rhs: DynamicSizedMatrix) -> Self::Output {
        rhs.mul(self)
    }
}

impl BitXor<DynamicSizedMatrix> for DynamicSizedMatrix {
    type Output = DynamicSizedMatrix;

    fn bitxor(self, rhs: DynamicSizedMatrix) -> Self::Output {
        let matrix_width = 0x1 << (self.qbit_size + rhs.qbit_size);
        let mut matrix = vec![Complex::zero(); matrix_width * matrix_width];

        for ax in 0..self.width {
            for ay in 0..self.width {
                let a = *self.matrix.get(ax * self.width + ay).unwrap();
                for bx in 0..rhs.width {
                    for by in 0..rhs.width {
                        let mx = ax + bx * self.width;
                        let my = ay + by * self.width;
                        *{ matrix.get_mut(mx * matrix_width + my).unwrap() } =
                            a * *{ rhs.matrix.get(bx * rhs.width + by).unwrap() };
                    }
                }
            }
        }

        DynamicSizedMatrix::new(matrix).unwrap()
    }
}

#[cfg(test)]
impl float_cmp::ApproxEq for DynamicSizedMatrix {
    type Margin = float_cmp::F64Margin;
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        let mut a = true;
        for x in 0..self.width {
            for y in 0..self.width {
                a = a && { *self.matrix.get(x * self.width + y).unwrap() }.approx_eq(
                    *other.matrix.get(x * self.width + y).unwrap(),
                    margin
                );
            }
        }

        a
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;
    use float_cmp::{ApproxEq, F64Margin};
    use crate::complex::Complex;
    use crate::runtime::dynamic_sized::matrix::DynamicSizedMatrix;

    #[test]
    fn test_multiply_x_x() {
        let a: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let b: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let c: DynamicSizedMatrix = a * b;
        let ce: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }

    #[test]
    fn test_multiply_c_x() {
        let a: Complex = Complex::new(0.0, -1.0);
        let b: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let c: DynamicSizedMatrix = a * b;
        let ce: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
            Complex::new(0.0, -1.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }

    #[test]
    fn test_tensordot_x_x() {
        let a: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let b: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let c: DynamicSizedMatrix = a ^ b;
        let ce: DynamicSizedMatrix = DynamicSizedMatrix::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }
}
