use alloc::{format, vec};
use alloc::vec::Vec;
use core::mem::size_of;
use core::ops::{BitXor, Mul, Not};

use crate::complex::Complex;
use crate::error::{QuantumError, Result};
use crate::quantum::computer::QuantumComputer;
use crate::quantum::ket::{IndexType, Ket};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
struct DGateDefinition(Vec<Complex>);

impl TryFrom<DGateDefinition> for DGate {
    type Error = QuantumError;
    fn try_from(value: DGateDefinition) -> Result<Self> {
        DGate::new(value.0)
    }
}

#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-pack", serde(try_from = "DGateDefinition"))]
#[derive(Clone, Debug, PartialEq)]
pub struct DGate {
    matrix: Vec<Complex>,
    width: usize,
    qbit_size: usize,
}

impl DGate {
    pub fn new(matrix: Vec<Complex>) -> Result<DGate> {
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
                return Ok(DGate {
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

    pub fn apply(&self, computer: &mut QuantumComputer, qbits: &[usize]) {
        // Assert all qbits exist and are different
        assert_eq!(self.qbit_size, qbits.len());
        for i in 0..qbits.len() {
            assert!(computer.get_state().size >= qbits[i]);
            for j in 0..qbits.len() {
                if i != j {
                    assert_ne!(qbits[i], qbits[j]);
                }
            }
        }

        let old_vec = &computer.get_state().vec;
        let mut new_ket = Ket::new_zero(computer.get_state().size);
        #[cfg(feature = "rayon")]
            let iter = new_ket.vec.par_iter_mut();
        #[cfg(not(feature = "rayon"))]
            let iter = new_ket.vec.iter_mut();
        iter.enumerate().for_each(|(ket_idx, new_ket_value)| {
            let mut ket_idx: IndexType = ket_idx.into();
            let mut y = IndexType::from(0);
            for pos in 0..self.qbit_size {
                #[cfg(all(not(test), not(feature = "safe")))]
                unsafe { y.set(pos, ket_idx.get(qbits[pos]).unwrap_unchecked()) };
                #[cfg(any(test, feature = "safe"))]
                y.set(pos, ket_idx.get(qbits[pos]).unwrap());
            }
            let y: usize = y.into();

            for x in 0..self.width {
                let x_idx: IndexType = x.into();
                for pos in 0..self.qbit_size {
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
                #[cfg(any(test, feature = "safe"))]
                let matrix_value = *{ self.matrix.get(x * self.width + y).unwrap() };
                #[cfg(all(not(test), not(feature = "safe")))]
                let matrix_value = *unsafe { self.matrix.get(x * self.width + y).unwrap_unchecked() };
                *new_ket_value = *new_ket_value + matrix_value * v;
            }
        });
        computer.set_state(new_ket);
    }
}

impl Mul for DGate {
    type Output = DGate;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix: Vec<Complex> = vec![Complex::zero(); self.matrix.len()];
        for x in 0..self.width {
            for y in 0..self.width {
                for i in 0..self.width {
                    #[cfg(all(not(test), not(feature = "safe")))]
                    {
                        *unsafe { matrix.get_unchecked_mut(x * self.width + y) } =
                            *unsafe { matrix.get_unchecked(x * self.width + y) }
                                + *unsafe { self.matrix.get_unchecked(i * self.width + y) }
                                * *unsafe { rhs.matrix.get_unchecked(x * self.width + i) };
                    }
                    #[cfg(any(test, feature = "safe"))]
                    {
                        *{ matrix.get_mut(x * self.width + y).unwrap() } =
                            *{ matrix.get(x * self.width + y).unwrap() }
                                + *{ self.matrix.get(i * self.width + y).unwrap() }
                                * *{ rhs.matrix.get(x * self.width + i).unwrap() };
                    }
                }
            }
        }

        #[cfg(all(not(test), not(feature = "safe")))]
        unsafe { DGate::new(matrix).unwrap_unchecked() }
        #[cfg(any(test, feature = "safe"))]
        DGate::new(matrix).unwrap()
    }
}

impl<T: Mul<Complex, Output = Complex> + Copy> Mul<T> for DGate {
    type Output = DGate;

    fn mul(self, rhs: T) -> Self::Output {
        let mut matrix: Vec<Complex> = vec![Complex::zero(); self.matrix.len()];
        for x in 0..self.width {
            for y in 0..self.width {
                #[cfg(all(not(test), not(feature = "safe")))]
                {
                    *unsafe { matrix.get_unchecked_mut(x * self.width + y) } =
                        rhs.mul(*unsafe { self.matrix.get_unchecked(x * self.width + y) });
                }
                #[cfg(any(test, feature = "safe"))]
                {
                    *{ matrix.get_mut(x * self.width + y).unwrap() } =
                        rhs.mul(*{ self.matrix.get(x * self.width + y).unwrap() });
                }
            }
        }

        #[cfg(all(not(test), not(feature = "safe")))]
        unsafe { DGate::new(matrix).unwrap_unchecked() }
        #[cfg(any(test, feature = "safe"))]
        DGate::new(matrix).unwrap()
    }
}

impl Mul<DGate> for Complex {
    type Output = DGate;

    #[inline(always)]
    fn mul(self, rhs: DGate) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<DGate> for f64 {
    type Output = DGate;

    #[inline(always)]
    fn mul(self, rhs: DGate) -> Self::Output {
        rhs.mul(self)
    }
}

impl BitXor<DGate> for DGate {
    type Output = DGate;

    fn bitxor(self, rhs: DGate) -> Self::Output {
        let matrix_width = 0x1 << (self.qbit_size + rhs.qbit_size);
        let mut matrix = vec![Complex::zero(); matrix_width * matrix_width];

        for ax in 0..self.width {
            for ay in 0..self.width {
                #[cfg(all(not(test), not(feature = "safe")))]
                    let a = *unsafe { self.matrix.get_unchecked(ax * self.width + ay) };
                #[cfg(any(test, feature = "safe"))]
                    let a = *self.matrix.get(ax * self.width + ay).unwrap();
                for bx in 0..rhs.width {
                    for by in 0..rhs.width {
                        let mx = ax + bx * self.width;
                        let my = ay + by * self.width;
                        #[cfg(all(not(test), not(feature = "safe")))]
                        {
                            *unsafe { matrix.get_unchecked_mut(mx * matrix_width + my) } =
                                a * *unsafe { rhs.matrix.get_unchecked(bx * rhs.width + by) };
                        }
                        #[cfg(any(test, feature = "safe"))]
                        {
                            *{ matrix.get_mut(mx * matrix_width + my).unwrap() } =
                                a * *{ rhs.matrix.get(bx * rhs.width + by).unwrap() };
                        }
                    }
                }
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
    use alloc::vec::Vec;
    use float_cmp::{ApproxEq, F64Margin};
    use crate::complex::Complex;
    use crate::quantum::gate::matrix::dynamic::DGate;
    use crate::quantum::operator::simple::controlled::Controlled;
    use crate::quantum::operator::simple::pauli_x::PauliX;
    use crate::quantum::operator::traits::ToGate;

    impl ApproxEq for DGate {
        type Margin = F64Margin;
        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let margin = margin.into();
            let mut a = true;
            for x in 0..self.width {
                for y in 0..self.width {
                    #[cfg(all(not(test), not(feature = "safe")))]
                    {
                        a = a && unsafe { *self.matrix.get_unchecked(x * self.width + y) }.approx_eq(
                            unsafe { *other.matrix.get_unchecked(x * self.width + y) },
                            margin
                        );
                    }
                    #[cfg(any(test, feature = "safe"))]
                    {
                        a = a && { *self.matrix.get(x * self.width + y).unwrap() }.approx_eq(
                            *other.matrix.get(x * self.width + y).unwrap(),
                            margin
                        );
                    }
                }
            }

            a
        }
    }

    #[test]
    fn test_multiply_x_x_dgate() {
        let a: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let b: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let c: DGate = a * b;
        let ce: DGate = DGate::new(Vec::from([
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }

    #[test]
    fn test_multiply_c_x_dgate() {
        let a: Complex = Complex::new(0.0, -1.0);
        let b: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let c: DGate = a * b;
        let ce: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
            Complex::new(0.0, -1.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }

    #[test]
    fn test_tensordot_x_x_dgate() {
        let a: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let b: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        let c: DGate = a ^ b;
        let ce: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }

    #[test]
    fn test_tensordot_x_cx_dgate() {
        let a: DGate = Controlled::<2, _>::new(0, PauliX::new(1)).to_gate().into();
        let b: DGate = PauliX::new(0).to_gate().into();
        let c: DGate = a ^ b;
        let ce: DGate = DGate::new(Vec::from([
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
        ])).unwrap();
        assert!(c.approx_eq(ce, F64Margin::zero().epsilon(0.00001)));
    }
}
