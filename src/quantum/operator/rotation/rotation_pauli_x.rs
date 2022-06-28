use core::f64::consts::PI;
use libm::{cos, sin};

use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_x::rx;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

pub fn rpx(theta: f64) -> Gate<1> {
    Complex::new(cos(theta / 2.0), sin(theta / 2.0)) * rx(theta)
}

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct RotationPauliX {
    wire: usize,
    theta: f64,
}

impl RotationPauliX {
    pub fn new(wire: usize, theta: f64) -> RotationPauliX {
        RotationPauliX { wire, theta }
    }
}

impl ToGate<1> for RotationPauliX {
    fn to_gate(&self) -> Gate<1> {
        rpx(self.theta)
    }
}

impl UsedWires<1> for RotationPauliX {
    fn wires(&self) -> [usize; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationPauliX {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, self.wires());
    }
}

impl Parameterized<1> for RotationPauliX {
    fn parameterized(&self) -> fn(&RotationPauliX, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationPauliX, theta: f64) -> Gate<1> {
            RotationPauliX::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}

#[cfg(test)]
mod test {
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::simple::pauli_x::PauliX;
    use crate::quantum::operator::simple::pauli_x_root::PauliXRoot;
    use crate::quantum::operator::rotation::rotation_pauli_x::RotationPauliX;
    use crate::quantum::operator::traits::ToGate;
    use core::f64::consts::PI;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_full() {
        let gate_rx = RotationPauliX::new(0, PI).to_gate();
        let gate_x = PauliX::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rx, gate_x, epsilon = 0.000001);
    }

    #[test]
    fn test_half() {
        let gate_rx = RotationPauliX::new(0, PI / 2.0).to_gate();
        let gate_x = PauliX::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rx * gate_rx, gate_x, epsilon = 0.000001);
    }

    #[test]
    fn test_third() {
        let gate_rx = RotationPauliX::new(0, PI / 3.0).to_gate();
        let gate_x = PauliX::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rx * gate_rx * gate_rx,
            gate_x,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_quarter() {
        let gate_rx = RotationPauliX::new(0, PI / 4.0).to_gate();
        let gate_x = PauliX::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rx * gate_rx * gate_rx * gate_rx,
            gate_x,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_full_root() {
        let gate_rx = RotationPauliX::new(0, PI / 2.0).to_gate();
        let gate_x = PauliXRoot::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rx, gate_x, epsilon = 0.000001);
    }

    #[test]
    fn test_half_root() {
        let gate_rx = RotationPauliX::new(0, PI / 4.0).to_gate();
        let gate_x = PauliXRoot::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rx * gate_rx, gate_x, epsilon = 0.000001);
    }

    #[test]
    fn test_third_root() {
        let gate_rx = RotationPauliX::new(0, PI / 6.0).to_gate();
        let gate_x = PauliXRoot::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rx * gate_rx * gate_rx,
            gate_x,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_quarter_root() {
        let gate_rx = RotationPauliX::new(0, PI / 8.0).to_gate();
        let gate_x = PauliXRoot::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rx * gate_rx * gate_rx * gate_rx,
            gate_x,
            epsilon = 0.000001
        );
    }
}
