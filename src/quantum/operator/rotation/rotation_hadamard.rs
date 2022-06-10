use crate::complex::Complex;
use core::f64::consts::SQRT_2;
use core::f64::consts::PI;
use libm::{cos, sin};

use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

static S: Gate<1> = Gate::new([
    [Complex::new(1.0 - SQRT_2, 0.0), Complex::new(1.0, 0.0)],
    [Complex::new(1.0 + SQRT_2, 0.0), Complex::new(1.0, 0.0)],
]);
static S_INV: Gate<1> = Gate::new([
    [
        Complex::new(-1.0 / (2.0 * SQRT_2), 0.0),
        Complex::new(1.0 / (2.0 * SQRT_2), 0.0),
    ],
    [
        Complex::new(0.25 * (2.0 + SQRT_2), 0.0),
        Complex::new(0.25 * (2.0 - SQRT_2), 0.0),
    ],
]);

pub fn rh(theta: f64) -> Gate<1> {
    S * Gate::new([
        [Complex::new(cos(theta), sin(theta)), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]) * S_INV
}

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct RotationHadamard {
    wire: u32,
    theta: f64,
}

impl RotationHadamard {
    pub fn new(wire: u32, theta: f64) -> RotationHadamard {
        RotationHadamard { wire, theta }
    }
}

impl ToGate<1> for RotationHadamard {
    fn to_gate(&self) -> Gate<1> {
        rh(self.theta)
    }
}

impl UsedWires<1> for RotationHadamard {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationHadamard {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, self.wires());
    }
}

impl Parameterized<1> for RotationHadamard {
    fn parameterized(&self) -> fn(&RotationHadamard, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationHadamard, theta: f64) -> Gate<1> {
            RotationHadamard::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}

#[cfg(test)]
mod test {
    use core::f64::consts::PI;
    use float_cmp::assert_approx_eq;

    use crate::quantum::operator::simple::hadamard::Hadamard;
    use crate::quantum::operator::rotation::rotation_hadamard::RotationHadamard;
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::traits::ToGate;

    #[test]
    fn test_full() {
        let gate_rh = RotationHadamard::new(0, PI).to_gate();
        let gate_h = Hadamard::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rh, gate_h, epsilon = 0.000001);
    }

    #[test]
    fn test_half() {
        let gate_rh = RotationHadamard::new(0, PI / 2.0).to_gate();
        let gate_h = Hadamard::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rh * gate_rh, gate_h, epsilon = 0.000001);
    }

    #[test]
    fn test_third() {
        let gate_rh = RotationHadamard::new(0, PI / 3.0).to_gate();
        let gate_h = Hadamard::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rh * gate_rh * gate_rh,
            gate_h,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_quarter() {
        let gate_rh = RotationHadamard::new(0, PI / 4.0).to_gate();
        let gate_h = Hadamard::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rh * gate_rh * gate_rh * gate_rh,
            gate_h,
            epsilon = 0.000001
        );
    }
}
