use core::f64::consts::SQRT_2;
use libm::sqrt;
use rand::RngCore;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::runtime::register::Register;
use crate::runtime::unitary::UnitaryOperator;
use crate::runtime::ket::Ket;
use crate::runtime::matrix::Matrix;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate)struct Measurement {
    wire: usize,
    basis: MeasurementBasis,
    creg_bit: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate)enum MeasurementBasis {
    X, Y, Z
}

impl Default for MeasurementBasis {
    fn default() -> Self {
        MeasurementBasis::Z
    }
}

impl Measurement {
    pub(crate) fn new(
        wire: usize,
        basis: Option<MeasurementBasis>,
        creg_bit: Option<usize>,
    ) -> Measurement {
        Measurement {
            wire,
            basis: basis.unwrap_or_default(),
            creg_bit,
        }
    }
}

static HADAMARD: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new(
    [
        [
            Complex::new(1.0 / SQRT_2, 0.0),
            Complex::new(1.0 / SQRT_2, 0.0),
        ],
        [
            Complex::new(1.0 / SQRT_2, 0.0),
            Complex::new(-1.0 / SQRT_2, 0.0),
        ],
    ]
);
static S_INV_GATE: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new(
    [
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
    ]
);

impl UnitaryOperator for Measurement {
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket {
        let transformed = match self.basis {
            MeasurementBasis::X => HADAMARD.apply(ket, &[ self.wire ]),
            MeasurementBasis::Y => HADAMARD.apply(S_INV_GATE.apply(ket, &[ self.wire ]), &[ self.wire ]),
            MeasurementBasis::Z => ket,
        };

        let probability: f64 = transformed.probability(self.wire);
        let random_weight: f64 = transformed.seed.lock().next_u32() as f64 / u32::MAX as f64;
        let state = probability - random_weight > 0.0;

        if let Some(bit) = &self.creg_bit {
            register.set(*bit, state);
        }

        let transformed = if state {
            if probability == 0.0 {
                ConstSizedMatrix::new([
                    [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
                ])
            } else {
                ConstSizedMatrix::new([
                    [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0), Complex::new(sqrt(1.0 / probability), 0.0)],
                ])
            }
        } else {
            if probability == 0.0 {
                ConstSizedMatrix::new([
                    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                ])
            } else {
                ConstSizedMatrix::new([
                    [Complex::new(sqrt(1.0 / probability), 0.0), Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                ])
            }
        }.apply(transformed, &[ self.wire ]);

        match self.basis {
            MeasurementBasis::X => HADAMARD.apply(transformed, &[ self.wire ]),
            MeasurementBasis::Y => HADAMARD.apply(S_INV_GATE.apply(transformed, &[ self.wire ]), &[ self.wire ]),
            MeasurementBasis::Z => transformed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;

    const CX: ConstSizedMatrix<2, Complex> = ConstSizedMatrix::new([
        [ Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), ],
        [ Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), ],
        [ Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), ],
        [ Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), ],
    ]);

    const PAULI_X: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]);

    #[test]
    fn test_measurement_bv() {
        let ket = Ket::new(5).unwrap();
        let mut register = Register::new(4);
        let ket = PAULI_X.apply(ket, &[4]);
        let ket = HADAMARD.apply(ket, &[4]);

        let ket = HADAMARD.apply(ket, &[0]);
        let ket = HADAMARD.apply(ket, &[1]);
        let ket = HADAMARD.apply(ket, &[2]);
        let ket = HADAMARD.apply(ket, &[3]);

        let ket = CX.apply(ket, &[0, 4]);
        let ket = CX.apply(ket, &[1, 4]);
        let ket = CX.apply(ket, &[3, 4]);

        let ket = HADAMARD.apply(ket, &[0]);
        let ket = HADAMARD.apply(ket, &[1]);
        let ket = HADAMARD.apply(ket, &[2]);
        let ket = HADAMARD.apply(ket, &[3]);
        let ket = HADAMARD.apply(ket, &[4]);

        let ket = Measurement::new(0, Some(MeasurementBasis::Z), Some(0)).apply(ket, &mut register);
        let ket = Measurement::new(1, Some(MeasurementBasis::Z), Some(1)).apply(ket, &mut register);
        let ket = Measurement::new(2, Some(MeasurementBasis::Z), Some(2)).apply(ket, &mut register);
        Measurement::new(3, Some(MeasurementBasis::Z), Some(3)).apply(ket, &mut register);

        assert_eq!(true, *register.get(0).unwrap());
        assert_eq!(true, *register.get(1).unwrap());
        assert_eq!(false, *register.get(2).unwrap());
        assert_eq!(true, *register.get(3).unwrap());
    }
}
