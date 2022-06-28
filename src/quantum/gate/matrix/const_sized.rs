use alloc::vec::Vec;
use core::ops::{BitXor, Mul};

use crate::complex::Complex;
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::matrix::dynamic::DGate;
use crate::quantum::ket::{IndexType, Ket};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Gate<const SIZE: usize>
    where
        [(); 0x1 << SIZE]:,
{
    // x y
    matrix: [[Complex; 0x1 << SIZE]; 0x1 << SIZE],
}

impl<const SIZE: usize> Gate<SIZE>
    where
        [(); 0x1 << SIZE]:,
{
    pub const fn new(matrix: [[Complex; 0x1 << SIZE]; 0x1 << SIZE]) -> Gate<SIZE> {
        Gate { matrix }
    }

    pub fn apply(&self, computer: &mut QuantumComputer, qbits: [usize; SIZE]) {
        // Assert all qbits exist and are different
        for i in 0..SIZE {
            assert!(computer.get_state().size >= qbits[i]);
            for j in 0..SIZE {
                if i != j {
                    assert_ne!(qbits[i], qbits[j]);
                }
            }
        }

        let old_vec = &computer.get_state().vec;
        let mut new_ket = Ket::new_zero(computer.get_state().size);
        if SIZE > 0 {
            #[cfg(feature = "rayon")]
                let iter = new_ket.vec.par_iter_mut();
            #[cfg(not(feature = "rayon"))]
                let iter = new_ket.vec.iter_mut();
            iter.enumerate().for_each(|(ket_idx, new_ket_value)| {
                let mut ket_idx: IndexType = ket_idx.into();
                let mut y = IndexType::from(0);
                for pos in 0..SIZE {
                    #[cfg(all(not(test), not(feature = "safe")))]
                    unsafe { y.set(pos, ket_idx.get(qbits[pos]).unwrap_unchecked()) };
                    #[cfg(any(test, feature = "safe"))]
                    y.set(pos, ket_idx.get(qbits[pos]).unwrap());
                }
                let y: usize = y.into();

                for x in 0..0x1 << SIZE {
                    let x_idx: IndexType = x.into();
                    for pos in 0..SIZE {
                        #[cfg(all(not(test), not(feature = "safe")))]
                        unsafe { ket_idx.set(qbits[pos], x_idx.get(pos).unwrap_unchecked()) };
                        #[cfg(any(test, feature = "safe"))]
                        ket_idx.set(qbits[pos], x_idx.get(pos).unwrap());
                    }
                    let ket_idx: usize = ket_idx.clone().into();

                    #[cfg(all(not(test), not(feature = "safe")))]
                    let v = *unsafe { old_vec.get(ket_idx).unwrap_unchecked() };
                    #[cfg(any(test, feature = "safe"))]
                    let v = *old_vec.get(ket_idx).unwrap();
                    *new_ket_value = *new_ket_value + self.matrix[x][y] * v;
                }
            });
        }

        computer.set_state(new_ket);
    }

    pub const fn matrix(&self) -> &[[Complex; 0x1 << SIZE]; 0x1 << SIZE] {
        return &self.matrix;
    }
}

impl<const SIZE_L: usize, const SIZE_R: usize> BitXor<Gate<SIZE_R>> for Gate<SIZE_L>
    where
        [(); 0x1 << SIZE_L]:,
        [(); 0x1 << SIZE_R]:,
        [(); 0x1 << (SIZE_L - 1)]:,
        [(); 0x1 << (SIZE_R - 1)]:,
        [(); 0x1 << (SIZE_L + SIZE_R)]:,
{
    type Output = Gate<{SIZE_L + SIZE_R}>;

    fn bitxor(self, rhs: Gate<SIZE_R>) -> Self::Output {
        let mut matrix = [[Complex::zero(); 0x1 << (SIZE_L + SIZE_R)]; 0x1 << (SIZE_L + SIZE_R)];

        for ax in 0..0x1 << SIZE_L {
            for ay in 0..0x1 << SIZE_L {
                let a = self.matrix[ax][ay];
                for bx in 0..0x1 << SIZE_R {
                    for by in 0..0x1 << SIZE_R {
                        matrix[ax + bx * (0x1 << SIZE_L)][ay + by * (0x1 << SIZE_L)] = a * rhs.matrix[bx][by];
                    }
                }
            }
        }

        Gate::new(matrix)
    }
}

impl<const SIZE: usize> Mul for Gate<SIZE>
    where
        [(); 0x1 << SIZE]:,
{
    type Output = Gate<SIZE>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut o: Gate<SIZE> = Gate::new([[Complex::new(0.0, 0.0); 0x1 << SIZE]; 0x1 << SIZE]);
        for x in 0..0x1 << SIZE {
            for y in 0..0x1 << SIZE {
                for i in 0..0x1 << SIZE {
                    o.matrix[x][y] = o.matrix[x][y] + self.matrix[i][y] * rhs.matrix[x][i];
                }
            }
        }

        o
    }
}

impl<const SIZE: usize, T: Mul<Complex, Output = Complex> + Copy> Mul<T> for Gate<SIZE>
    where
        [(); 0x1 << SIZE]:,
{
    type Output = Gate<SIZE>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut o: Gate<SIZE> = Gate::new([[Complex::new(0.0, 0.0); 0x1 << SIZE]; 0x1 << SIZE]);
        for x in 0..0x1 << SIZE {
            for y in 0..0x1 << SIZE {
                o.matrix[x][y] = rhs.mul(self.matrix[x][y]);
            }
        }

        o
    }
}

impl<const SIZE: usize> Mul<Gate<SIZE>> for Complex
    where
        [(); 0x1 << SIZE]:,
{
    type Output = Gate<SIZE>;

    #[inline(always)]
    fn mul(self, rhs: Gate<SIZE>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<const SIZE: usize> Mul<Gate<SIZE>> for f64
    where
        [(); 0x1 << SIZE]:,
{
    type Output = Gate<SIZE>;

    #[inline(always)]
    fn mul(self, rhs: Gate<SIZE>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<const SIZE: usize> Into<DGate> for Gate<SIZE>
    where
        [(); 0x1 << SIZE]:,
{
    fn into(self) -> DGate {
        let mut matrix: Vec<Complex> = Vec::with_capacity(0x1 << (SIZE * 2));
        for x in 0..0x1 << SIZE {
            for y in 0..0x1 << SIZE {
                matrix.push(self.matrix[x][y]);
            }
        }

        #[cfg(all(not(test), not(feature = "safe")))]
        unsafe { DGate::new(matrix).unwrap_unchecked() }
        #[cfg(any(test, feature = "safe"))]
        DGate::new(matrix).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::complex::Complex;
    use float_cmp::assert_approx_eq;
    use float_cmp::{ApproxEq, F64Margin};
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::simple::controlled::Controlled;
    use crate::quantum::operator::simple::pauli_x::PauliX;
    use crate::quantum::operator::traits::ToGate;

    impl<const SIZE: usize> ApproxEq for Gate<SIZE>
        where
            [(); 0x1 << SIZE]:,
    {
        type Margin = F64Margin;
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

    #[test]
    fn test_multiply_x_x() {
        let a: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: Gate<1> = a * b;
        let ce: Gate<1> = Gate::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]);
        assert_approx_eq!(Gate<1>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_x_i() {
        let a: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: Gate<1> = Gate::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]);
        let c: Gate<1> = a * b;
        let ce: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(Gate<1>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_x_c() {
        let a: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: Complex = Complex::new(0.0, -1.0);
        let c: Gate<1> = a * b;
        let ce: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, -1.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(Gate<1>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_multiply_c_x() {
        let a: Complex = Complex::new(0.0, -1.0);
        let b: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: Gate<1> = a * b;
        let ce: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, -1.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(Gate<1>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_tensordot_x_x() {
        let a: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let b: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: Gate<2> = a ^ b;
        let ce: Gate<2> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(Gate<2>, c, ce, epsilon = 0.00001);
    }

    #[test]
    fn test_tensordot_x_cx() {
        let a: Gate<2> = Controlled::new(0, PauliX::new(1)).to_gate();
        let b: Gate<1> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        let c: Gate<3> = a ^ b;
        let ce: Gate<3> = Gate::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        ]);
        assert_approx_eq!(Gate<3>, c, ce, epsilon = 0.00001);
    }
}
