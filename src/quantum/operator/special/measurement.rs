use libm::sqrt;
use rand::RngCore;
use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

pub static MEASUREMENT: Gate<1> = Gate::new([
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;
use crate::quantum::operator::simple::hadamard::HADAMARD;
use crate::quantum::operator::simple::s_gate::S_GATE;
use crate::quantum::operator::simple::s_gate_inverse::S_INV_GATE;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct Measurement {
    wire: u32,

    #[cfg_attr(feature = "wasm-pack", tsify(optional))]
    #[cfg_attr(feature = "wasm-pack", serde(default))]
    basis: MeasurementBasis,

    #[cfg_attr(feature = "wasm-pack", tsify(optional))]
    creg: Option<u32>,

    #[cfg_attr(feature = "wasm-pack", tsify(optional))]
    creg_bit: Option<u32>,
}

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub enum MeasurementBasis {
    X, Y, Z
}

impl Default for MeasurementBasis {
    fn default() -> Self {
        MeasurementBasis::Z
    }
}

impl Measurement {
    pub fn new(
        wire: u32,
        basis: Option<MeasurementBasis>,
        creg: Option<u32>,
        creg_bit: Option<u32>,
    ) -> Measurement {
        Measurement {
            wire,
            basis: basis.unwrap_or_default(),
            creg,
            creg_bit,
        }
    }
}

impl ApplyGate<1> for Measurement {
    fn apply(&self, computer: &mut QuantumComputer) {
        match self.basis {
            MeasurementBasis::X => HADAMARD.apply(computer, [ self.wire ]),
            MeasurementBasis::Y => {
                S_INV_GATE.apply(computer, [ self.wire ]);
                HADAMARD.apply(computer, [ self.wire ]);
            },
            MeasurementBasis::Z => {},
        }

        let proballily: f64 = computer.probability(self.wire);
        let random_weight: f64 = 1.0 / (computer.seed.next_u64() as f64 + 1.0);
        let state = proballily - random_weight > 0.0;

        if state {
            Gate::new([
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(sqrt(1.0 / proballily), 0.0)],
            ])
        } else {
            Gate::new([
                [Complex::new(sqrt(1.0 / proballily), 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            ])
        }.apply(computer, self.wires());

        match self.basis {
            MeasurementBasis::X => HADAMARD.apply(computer, [ self.wire ]),
            MeasurementBasis::Y => {
                HADAMARD.apply(computer, [ self.wire ]);
                S_GATE.apply(computer, [ self.wire ]);
            },
            MeasurementBasis::Z => {},
        }
    }
}

impl UsedWires<1> for Measurement {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

#[cfg(test)]
mod test {
    use core::f64::consts::SQRT_2;
    use crate::quantum::operator::traits::ApplyGate;
    use float_cmp::assert_approx_eq;
    use crate::complex::Complex;
    use crate::quantum::computer::QuantumComputer;
    use crate::quantum::operator::simple::hadamard::Hadamard;
    use crate::quantum::operator::special::measurement::{Measurement, MeasurementBasis};

    #[test]
    fn test_measure_z() {
        let mut computer = QuantumComputer::new(2, Some(42));
        Hadamard::new(0).apply(&mut computer);
        Hadamard::new(1).apply(&mut computer);
        let state = &computer.get_state().vec;
        assert_approx_eq!(Complex, *state.get(0).unwrap(), Complex::new(0.5, 0.0), epsilon = 0.00001);
        assert_approx_eq!(Complex, *state.get(1).unwrap(), Complex::new(0.5, 0.0), epsilon = 0.00001);
        assert_approx_eq!(Complex, *state.get(2).unwrap(), Complex::new(0.5, 0.0), epsilon = 0.00001);
        assert_approx_eq!(Complex, *state.get(3).unwrap(), Complex::new(0.5, 0.0), epsilon = 0.00001);
        Measurement::new(0, Some(MeasurementBasis::Z), None, None).apply(&mut computer);
        let state = &computer.get_state().vec;
        assert_approx_eq!(Complex, *state.get(0).unwrap(), Complex::zero(), epsilon = 0.00001);
        assert_approx_eq!(Complex, *state.get(1).unwrap(), Complex::new(1.0 / SQRT_2, 0.0), epsilon = 0.00001);
        assert_approx_eq!(Complex, *state.get(2).unwrap(), Complex::zero(), epsilon = 0.00001);
        assert_approx_eq!(Complex, *state.get(3).unwrap(), Complex::new(1.0 / SQRT_2, 0.0), epsilon = 0.00001);
    }
}