use alloc::vec::Vec;
use core::ops::{BitXor, Mul, Not};

use crate::complex::Complex;
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::matrix::dynamic::DGate;
use crate::quantum::ket::Ket;

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

    pub fn apply(&self, computer: &mut QuantumComputer, qbits: [u32; SIZE]) {
        // Assert all qbits exist and are different
        for i in 0..SIZE {
            assert!(computer.get_state().size >= qbits[i]);
            for j in 0..SIZE {
                if i != j {
                    assert_ne!(qbits[i], qbits[j]);
                }
            }
        }

        if SIZE > 0 {
            let mut new_ket = Ket::new_zero(computer.get_state().size);
            for i in 0_usize..computer.get_state().vec.len() {
                #[cfg(not(test))]
                    let v = *unsafe { computer.get_state().vec.get_unchecked(i) };
                #[cfg(test)]
                    let v = *computer.get_state().vec.get(i).unwrap();

                let mut x: usize = 0;
                for pos in 0..SIZE {
                    x |= ((i & (0x1_usize << qbits[pos]) > 0) as usize) << pos;
                }
                // set all qbit indexes to 0
                let mut ir = i;
                for pos in 0..SIZE {
                    ir &= (0x1_usize << qbits[pos]).not()
                }
                for y in 0..0x1 << SIZE {
                    let mut it = ir;
                    for pos in 0..SIZE {
                        it |= ((y & (0x1_usize << pos) > 0) as usize) << qbits[pos];
                    }
                    #[cfg(not(test))]
                    {
                        *unsafe { new_ket.vec.get_unchecked_mut(it) } =
                            *unsafe { new_ket.vec.get_unchecked(it) } + self.matrix[x][y] * v;
                    }
                    #[cfg(test)]
                    {
                        *new_ket.vec.get_mut(it).unwrap() =
                            *new_ket.vec.get(it).unwrap() + self.matrix[x][y] * v;
                    }
                }
            }
            computer.set_state(new_ket);
        }
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

        #[cfg(not(test))]
        unsafe { DGate::new(matrix).unwrap_unchecked() }
        #[cfg(test)]
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
