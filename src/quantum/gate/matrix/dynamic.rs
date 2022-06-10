use alloc::{format, vec};
use alloc::vec::Vec;
use core::mem::size_of;
use core::ops::{BitXor, Mul, Not};

use crate::complex::Complex;
use crate::error::{QuantumError, Result};
use crate::quantum::computer::QuantumComputer;
use crate::quantum::ket::Ket;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
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
    qbit_size: u32,
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
                    qbit_size: (i / 2) as u32,
                });
            }
        }

        Err(QuantumError(
            format!("Expecting matrix length to be a square root of a power of two but was {}", len)
        ))
    }

    pub fn apply(&self, computer: &mut QuantumComputer, qbits: &[u32]) {
        // Assert all qbits exist and are different
        assert_eq!(self.qbit_size, qbits.len() as u32);
        for i in 0..qbits.len() {
            assert!(computer.get_state().size >= qbits[i]);
            for j in 0..qbits.len() {
                if i != j {
                    assert_ne!(qbits[i], qbits[j]);
                }
            }
        }

        let mut new_ket = Ket::new_zero(computer.get_state().size);
        for i in 0_usize..computer.get_state().vec.len() {
            #[cfg(not(test))]
                let v = *unsafe { computer.get_state().vec.get_unchecked(i) };
            #[cfg(test)]
                let v = *computer.get_state().vec.get(i).unwrap();
            let mut x: usize = 0;
            for pos in 0..qbits.len() {
                x |= ((i & (0x1_usize << qbits[pos]) > 0) as usize) << pos;
            }
            // set all qbit indexes to 0
            let mut ir = i;
            for pos in 0..qbits.len() {
                ir &= (0x1_usize << qbits[pos]).not()
            }
            for y in 0..0x1 << qbits.len() {
                let mut it = ir;
                for pos in 0..qbits.len() {
                    it |= ((y & (0x1_usize << pos) > 0) as usize) << qbits[pos];
                }
                #[cfg(not(test))]
                {
                    *unsafe { new_ket.vec.get_unchecked_mut(it) } =
                        *unsafe { new_ket.vec.get_unchecked(it) }
                            + *unsafe { self.matrix.get_unchecked(x * self.width + y) } * v;
                }
                #[cfg(test)]
                {
                    *{ new_ket.vec.get_mut(it).unwrap() } =
                        *{ new_ket.vec.get(it).unwrap() }
                            + *{ self.matrix.get(x * self.width + y).unwrap() } * v;
                }
            }
        }

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
                    #[cfg(not(test))]
                    {
                        *unsafe { matrix.get_unchecked_mut(x * self.width + y) } =
                            *unsafe { matrix.get_unchecked(x * self.width + y) }
                                + *unsafe { self.matrix.get_unchecked(i * self.width + y) }
                                * *unsafe { rhs.matrix.get_unchecked(x * self.width + i) };
                    }
                    #[cfg(test)]
                    {
                        *{ matrix.get_mut(x * self.width + y).unwrap() } =
                            *{ matrix.get(x * self.width + y).unwrap() }
                                + *{ self.matrix.get(i * self.width + y).unwrap() }
                                * *{ rhs.matrix.get(x * self.width + i).unwrap() };
                    }
                }
            }
        }

        #[cfg(not(test))]
        unsafe { DGate::new(matrix).unwrap_unchecked() }
        #[cfg(test)]
        DGate::new(matrix).unwrap()
    }
}

impl<T: Mul<Complex, Output = Complex> + Copy> Mul<T> for DGate {
    type Output = DGate;

    fn mul(self, rhs: T) -> Self::Output {
        let mut matrix: Vec<Complex> = vec![Complex::zero(); self.matrix.len()];
        for x in 0..self.width {
            for y in 0..self.width {
                #[cfg(not(test))]
                {
                    *unsafe { matrix.get_unchecked_mut(x * self.width + y) } =
                        rhs.mul(*unsafe { self.matrix.get_unchecked(x * self.width + y) });
                }
                #[cfg(test)]
                {
                    *{ matrix.get_mut(x * self.width + y).unwrap() } =
                        rhs.mul(*{ self.matrix.get(x * self.width + y).unwrap() });
                }
            }
        }

        #[cfg(not(test))]
        unsafe { DGate::new(matrix).unwrap_unchecked() }
        #[cfg(test)]
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
                #[cfg(not(test))]
                    let a = *unsafe { self.matrix.get_unchecked(ax * self.width + ay) };
                #[cfg(test)]
                    let a = *self.matrix.get(ax * self.width + ay).unwrap();
                for bx in 0..rhs.width {
                    for by in 0..rhs.width {
                        let mx = ax + bx * self.width;
                        let my = ay + by * self.width;
                        #[cfg(not(test))]
                        {
                            *unsafe { matrix.get_unchecked_mut(mx * matrix_width + my) } =
                                a * *unsafe { rhs.matrix.get_unchecked(bx * rhs.width + by) };
                        }
                        #[cfg(test)]
                        {
                            *{ matrix.get_mut(mx * matrix_width + my).unwrap() } =
                                a * *{ rhs.matrix.get(bx * rhs.width + by).unwrap() };
                        }
                    }
                }
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
                    #[cfg(not(test))]
                    {
                        a = a && unsafe { *self.matrix.get_unchecked(x * self.width + y) }.approx_eq(
                            unsafe { *other.matrix.get_unchecked(x * self.width + y) },
                            margin
                        );
                    }
                    #[cfg(test)]
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
