use core::f64::consts::PI;
use libm::{cos, sin};

use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_y::ry;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

pub fn rpy(theta: f64) -> Gate<1> {
    Complex::new(cos(theta / 2.0), sin(theta / 2.0)) * ry(theta)
}

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct RotationPauliY {
    wire: u32,
    theta: f64,
}

impl RotationPauliY {
    pub fn new(wire: u32, theta: f64) -> RotationPauliY {
        RotationPauliY { wire, theta }
    }
}

impl ToGate<1> for RotationPauliY {
    fn to_gate(&self) -> Gate<1> {
        rpy(self.theta)
    }
}

impl UsedWires<1> for RotationPauliY {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationPauliY {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, [self.wire]);
    }
}

impl Parameterized<1> for RotationPauliY {
    fn parameterized(&self) -> fn(&RotationPauliY, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationPauliY, theta: f64) -> Gate<1> {
            RotationPauliY::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}

#[cfg(test)]
mod test {
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::simple::pauli_y::PauliY;
    use crate::quantum::operator::rotation::rotation_pauli_y::RotationPauliY;
    use crate::quantum::operator::traits::ToGate;
    use core::f64::consts::PI;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_full() {
        let gate_ry = RotationPauliY::new(0, PI).to_gate();
        let gate_y = PauliY::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_ry, gate_y, epsilon = 0.000001);
    }

    #[test]
    fn test_half() {
        let gate_ry = RotationPauliY::new(0, PI / 2.0).to_gate();
        let gate_y = PauliY::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_ry * gate_ry, gate_y, epsilon = 0.000001);
    }

    #[test]
    fn test_third() {
        let gate_ry = RotationPauliY::new(0, PI / 3.0).to_gate();
        let gate_y = PauliY::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_ry * gate_ry * gate_ry,
            gate_y,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_quarter() {
        let gate_ry = RotationPauliY::new(0, PI / 4.0).to_gate();
        let gate_y = PauliY::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_ry * gate_ry * gate_ry * gate_ry,
            gate_y,
            epsilon = 0.000001
        );
    }
}
