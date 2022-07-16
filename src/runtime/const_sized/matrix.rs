use core::fmt::{Display, Formatter};
use core::ops::{BitXor, Mul, Add};

use crate::complex::Complex;
use crate::runtime::ket::{IndexType, Ket};
use crate::util::const_iter::ConstIter;
use crate::runtime::matrix::Matrix;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate)struct ConstSizedMatrix<const SIZE: usize, T: Sized>
    where
        [(); 0x1 << SIZE]:,
{
    // x y
    matrix: [[T; 0x1 << SIZE]; 0x1 << SIZE],
}

impl<const SIZE: usize, T: Copy + Sized> ConstSizedMatrix<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    pub(crate) const fn new(matrix: [[T; 0x1 << SIZE]; 0x1 << SIZE]) -> ConstSizedMatrix<SIZE, T> {
        ConstSizedMatrix { matrix }
    }

    pub(crate)const fn matrix(&self) -> &[[T; 0x1 << SIZE]; 0x1 << SIZE] {
        return &self.matrix;
    }
}

pub(crate) trait Transpose<T> {
    fn transpose(&self) -> T;
}

impl<const SIZE: usize, T: Copy + Sized + ~const Default> const Transpose<ConstSizedMatrix<SIZE, T>> for ConstSizedMatrix<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    fn transpose(&self) -> ConstSizedMatrix<SIZE, T> {
        let mut matrix = [[T::default(); 0x1 << SIZE]; 0x1 << SIZE];
        for y in ConstIter(0, 0x1 << SIZE) {
            for x in ConstIter(0, 0x1 << SIZE) {
                matrix[x][y] = self.matrix[y][x];
            }
        }

        ConstSizedMatrix::new(matrix)
    }
}

impl<const SIZE: usize, T: Copy + Sized + ~const Default> const Default for ConstSizedMatrix<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    fn default() -> Self {
        ConstSizedMatrix::new([[T::default(); 0x1 << SIZE]; 0x1 << SIZE])
    }
}

impl<const SIZE: usize> ConstSizedMatrix<SIZE, Complex>
    where
        [(); 0x1 << SIZE]:,
{
    pub(crate)const fn conjugate(&self) -> ConstSizedMatrix<SIZE, Complex> {
        let mut matrix = [[Complex::default(); 0x1 << SIZE]; 0x1 << SIZE];
        for y in ConstIter(0, 0x1 << SIZE) {
            for x in ConstIter(0, 0x1 << SIZE) {
                matrix[y][x] = self.matrix[y][x].conjugate();
            }
        }

        ConstSizedMatrix::new(matrix)
    }
}

impl<
    const SIZE: usize,
    T: Mul<Complex, Output = Complex> + Copy + Sync + Send
> Matrix<SIZE> for ConstSizedMatrix<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    fn apply(&self, ket: Ket, wires: &[usize; SIZE]) -> Ket {
        // Assert all qbits exist and are different
        for i in 0..SIZE {
            assert!(ket.size() >= wires[i]);
            for j in 0..SIZE {
                if i != j {
                    assert_ne!(wires[i], wires[j]);
                }
            }
        }

        if SIZE > 0 {
            if SIZE == 1 {
                let mut ket = ket;
                #[cfg(feature = "rayon")]
                {
                    ket.vec.par_chunks_mut(0x1 << (wires[0] + 1)).flat_map_iter(|e| {
                        let (z, o) = e.split_at_mut(0x1 << wires[0]);

                        z.into_iter().zip(o)
                    }).for_each(|(z, o)| {
                        let zt = self.matrix[0][0] * *z + self.matrix[0][1] * *o;
                        let ot = self.matrix[1][0] * *z + self.matrix[1][1] * *o;

                        *z = zt;
                        *o = ot;
                    });
                }
                #[cfg(not(feature = "rayon"))]
                {
                    ket.vec.chunks_mut(0x1 << (wires[0] + 1)).flat_map(|e| {
                        let (z, o) = e.split_at_mut(0x1 << wires[0]);

                        z.into_iter().zip(o)
                    }).for_each(|(z, o)| {
                        let zt = self.matrix[0][0] * *z + self.matrix[0][1] * *o;
                        let ot = self.matrix[1][0] * *z + self.matrix[1][1] * *o;

                        *z = zt;
                        *o = ot;
                    });
                }

                ket
            } else {
                let old_vec = &ket.vec;
                let mut new_ket = Ket::new_from(&ket);
                #[cfg(feature = "rayon")]
                    let iter = new_ket.vec.par_iter_mut();
                #[cfg(not(feature = "rayon"))]
                    let iter = new_ket.vec.iter_mut();
                iter.enumerate().for_each(|(ket_idx, new_ket_value)| {
                    let mut ket_idx: IndexType = ket_idx.into();
                    let mut y = IndexType::from(0);
                    for pos in 0..SIZE {
                        y.set(pos, ket_idx.get(wires[SIZE - 1 - pos]).unwrap());
                    }
                    let y: usize = y.into();

                    for x in 0..0x1 << SIZE {
                        let x_idx: IndexType = x.into();
                        for pos in 0..SIZE {
                            ket_idx.set(wires[SIZE - 1 - pos], x_idx.get(pos).unwrap());
                        }
                        let ket_idx: usize = ket_idx.clone().into();

                        let v = *old_vec.get(ket_idx).unwrap();
                        *new_ket_value = *new_ket_value + self.matrix[y][x] * v;
                    }
                });

                new_ket
            }
        } else {
            ket
        }
    }
}

impl<
    const SIZE_L: usize,
    const SIZE_R: usize,
    L: ~const Mul<R, Output = O> + Copy + Sized + Sync + Send,
    R: Copy + Sized + Sync + Send,
    O: Copy + Sized + ~const Default + Sync + Send,
> const BitXor<ConstSizedMatrix<SIZE_R, R>> for ConstSizedMatrix<SIZE_L, L>
    where
        [(); 0x1 << SIZE_L]:,
        [(); 0x1 << SIZE_R]:,
        [(); 0x1 << (SIZE_L - 1)]:,
        [(); 0x1 << (SIZE_R - 1)]:,
        [(); 0x1 << (SIZE_L + SIZE_R)]:,
{
    type Output = ConstSizedMatrix<{SIZE_L + SIZE_R}, O>;

    fn bitxor(self, rhs: ConstSizedMatrix<SIZE_R, R>) -> Self::Output {
        let mut matrix = [[O::default(); 0x1 << (SIZE_L + SIZE_R)]; 0x1 << (SIZE_L + SIZE_R)];

        for ax in ConstIter(0, 0x1 << SIZE_L) {
            for ay in ConstIter(0, 0x1 << SIZE_L) {
                let a = self.matrix[ax][ay];
                for bx in ConstIter(0, 0x1 << SIZE_R) {
                    for by in ConstIter(0, 0x1 << SIZE_R) {
                        matrix[ax + bx * (0x1 << SIZE_L)][ay + by * (0x1 << SIZE_L)] = a * rhs.matrix[bx][by];
                    }
                }
            }
        }

        ConstSizedMatrix::new(matrix)
    }
}

impl<
    const SIZE: usize,
    L: ~const Mul<f64, Output = O> + Copy + Sized + Sync + Send,
    O: Copy + Sized + ~const Default + Sync + Send,
> const Mul<f64> for ConstSizedMatrix<SIZE, L>
    where
        [(); 0x1 << SIZE]:,
{
    type Output = ConstSizedMatrix<SIZE, O>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut o: ConstSizedMatrix<SIZE, O> = ConstSizedMatrix::default();
        for y in ConstIter(0, 0x1 << SIZE) {
            for x in ConstIter(0, 0x1 << SIZE) {
                o.matrix[y][x] = self.matrix[y][x] * rhs;
            }
        }

        o
    }
}

impl<
    const SIZE: usize,
    L: ~const Mul<Complex, Output = O> + Copy + Sized + Sync + Send,
    O: Copy + Sized + ~const Default + Sync + Send,
> const Mul<Complex> for ConstSizedMatrix<SIZE, L>
    where
        [(); 0x1 << SIZE]:,
{
    type Output = ConstSizedMatrix<SIZE, O>;

    fn mul(self, rhs: Complex) -> Self::Output {
        let mut o: ConstSizedMatrix<SIZE, O> = ConstSizedMatrix::default();
        for y in ConstIter(0, 0x1 << SIZE) {
            for x in ConstIter(0, 0x1 << SIZE) {
                o.matrix[y][x] = self.matrix[y][x] * rhs;
            }
        }

        o
    }
}

impl<
    const SIZE: usize,
    R: ~const Mul<f64, Output = O> + Copy + Sized + Sync + Send,
    O: Copy + Sized + ~const Default + Sync + Send,
> const Mul<ConstSizedMatrix<SIZE, R>> for f64
    where
        [(); 0x1 << SIZE]:,
{
    type Output = ConstSizedMatrix<SIZE, O>;

    fn mul(self, rhs: ConstSizedMatrix<SIZE, R>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<
    const SIZE: usize,
    R: ~const Mul<Complex, Output = O> + Copy + Sized + Sync + Send,
    O: Copy + Sized + ~const Default + Sync + Send,
> const Mul<ConstSizedMatrix<SIZE, R>> for Complex
    where
        [(); 0x1 << SIZE]:,
{
    type Output = ConstSizedMatrix<SIZE, O>;

    fn mul(self, rhs: ConstSizedMatrix<SIZE, R>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<
    const SIZE: usize,
    L: ~const Mul<R, Output = O> + Copy + Sized + Sync + Send,
    R: Copy + Sized + Sync + Send,
    O: ~const Add<O, Output = O> + Copy + Sized + ~const Default + Sync + Send,
> const Mul<ConstSizedMatrix<SIZE, R>> for ConstSizedMatrix<SIZE, L>
    where
        [(); 0x1 << SIZE]:,
{
    type Output = ConstSizedMatrix<SIZE, O>;

    fn mul(self, rhs: ConstSizedMatrix<SIZE, R>) -> Self::Output {
        let mut o: ConstSizedMatrix<SIZE, O> = ConstSizedMatrix::default();
        for y in ConstIter(0, 0x1 << SIZE) {
            for x in ConstIter(0, 0x1 << SIZE) {
                for i in ConstIter(0, 0x1 << SIZE) {
                    o.matrix[y][x] = o.matrix[y][x] + self.matrix[y][i] * rhs.matrix[i][x];
                }
            }
        }

        o
    }
}

impl<const SIZE: usize, T: Sized + Copy + Display> Display for ConstSizedMatrix<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for y in 0..0x1 << SIZE {
            write!(f, "| ")?;
            for x in 0..0x1 << SIZE {
                write!(f, "{}, ", self.matrix[y][x])?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}

#[cfg(test)]
impl<const SIZE: usize, T: Sized + Copy + float_cmp::ApproxEq<Margin = float_cmp::F64Margin>> float_cmp::ApproxEq for ConstSizedMatrix<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    type Margin = float_cmp::F64Margin;
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        let mut a = true;
        for x in 0..0x1 << SIZE {
            for y in 0..0x1 << SIZE {
                a = a && self.matrix[x][y].approx_eq(other.matrix[x][y], margin);
            }
        }

        a
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_multiply_x_x() {
        let a: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: ConstSizedMatrix<1, Complex> = a * b;
        let ce: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_x_i() {
        let a: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]);
        let c: ConstSizedMatrix<1, Complex> = a * b;
        let ce: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_x_c() {
        let a: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: Complex = Complex::new(0.0, -1.0);
        let c: ConstSizedMatrix<1, Complex> = a * b;
        let ce: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, -1.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_c_x_complex() {
        let a: Complex = Complex::new(0.0, -1.0);
        let b: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: ConstSizedMatrix<1, Complex> = a * b;
        let ce: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, -1.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_c_x_f64() {
        let a: Complex = Complex::new(0.0, -1.0);
        let b: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
            [0.0, 1.0],
            [1.0, 0.0],
        ]);
        let c: ConstSizedMatrix<1, Complex> = a * b;
        let ce: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, -1.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_tensordot_x_x_f64() {
        let a: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
            [0.0, 1.0],
            [1.0, 0.0],
        ]);
        let b: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
            [0.0, 1.0],
            [1.0, 0.0],
        ]);
        let c: ConstSizedMatrix<2, f64> = a ^ b;
        let ce: ConstSizedMatrix<2, f64> = ConstSizedMatrix::new([
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        ]);
        assert_approx_eq!(ConstSizedMatrix<2, f64>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_tensordot_x_x_complex() {
        let a: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: ConstSizedMatrix<2, Complex> = a ^ b;
        let ce: ConstSizedMatrix<2, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<2, Complex>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_tensordot_x_cx() {
        let a: ConstSizedMatrix<2, Complex> = ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: ConstSizedMatrix<3, Complex> = a ^ b;
        let ce: ConstSizedMatrix<3, Complex> = ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(ConstSizedMatrix<3, Complex>, c, ce, epsilon = 0.00001);
    }
}
