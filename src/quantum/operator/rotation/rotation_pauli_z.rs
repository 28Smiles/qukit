use core::f64::consts::PI;
use libm::{cos, sin};

use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_z::rz;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

pub fn rpy(phi: f64) -> Gate<1> {
    Complex::new(cos(phi / 2.0), sin(phi / 2.0)) * rz(phi)
}

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct RotationPauliZ {
    wire: u32,
    theta: f64,
}

impl RotationPauliZ {
    pub fn new(wire: u32, theta: f64) -> RotationPauliZ {
        RotationPauliZ { wire, theta }
    }
}

impl ToGate<1> for RotationPauliZ {
    fn to_gate(&self) -> Gate<1> {
        rpy(self.theta)
    }
}

impl UsedWires<1> for RotationPauliZ {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationPauliZ {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, [self.wire]);
    }
}

impl Parameterized<1> for RotationPauliZ {
    fn parameterized(&self) -> fn(&RotationPauliZ, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationPauliZ, theta: f64) -> Gate<1> {
            RotationPauliZ::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}

#[cfg(test)]
mod test {
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::simple::pauli_z::PauliZ;
    use crate::quantum::operator::rotation::rotation_pauli_z::RotationPauliZ;
    use crate::quantum::operator::traits::ToGate;
    use core::f64::consts::PI;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_full() {
        let gate_rz = RotationPauliZ::new(0, PI).to_gate();
        let gate_z = PauliZ::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rz, gate_z, epsilon = 0.000001);
    }

    #[test]
    fn test_half() {
        let gate_rz = RotationPauliZ::new(0, PI / 2.0).to_gate();
        let gate_z = PauliZ::new(0).to_gate();
        assert_approx_eq!(Gate<1>, gate_rz * gate_rz, gate_z, epsilon = 0.000001);
    }

    #[test]
    fn test_third() {
        let gate_rz = RotationPauliZ::new(0, PI / 3.0).to_gate();
        let gate_z = PauliZ::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rz * gate_rz * gate_rz,
            gate_z,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_quarter() {
        let gate_rz = RotationPauliZ::new(0, PI / 4.0).to_gate();
        let gate_z = PauliZ::new(0).to_gate();
        assert_approx_eq!(
            Gate<1>,
            gate_rz * gate_rz * gate_rz * gate_rz,
            gate_z,
            epsilon = 0.000001
        );
    }
}
