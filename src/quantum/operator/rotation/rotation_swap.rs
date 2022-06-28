use core::f64::consts::PI;
use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;
use libm::{cos, sin};

pub static S: Gate<2> = Gate::new([
    [
        Complex::new(0.0, 0.0),
        Complex::new(-1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
    ],
    [
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ],
]);
pub static S_INV: Gate<2> = Gate::new([
    [
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
    ],
    [
        Complex::new(-0.5, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.5, 0.0),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(0.5, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.5, 0.0),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ],
]);

pub fn rswap(theta: f64) -> Gate<2> {
    S * Gate::new([
        [
            Complex::new(cos(theta), sin(theta)),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ],
        [
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ],
        [
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ],
        [
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
        ],
    ]) * S_INV
}

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct RotationSwap {
    wire_0: usize,
    wire_1: usize,
    theta: f64,
}

impl RotationSwap {
    pub fn new(wire_0: usize, wire_1: usize, theta: f64) -> RotationSwap {
        RotationSwap {
            wire_0,
            wire_1,
            theta,
        }
    }
}

impl ToGate<2> for RotationSwap {
    fn to_gate(&self) -> Gate<2> {
        rswap(self.theta)
    }
}

impl ApplyGate<2> for RotationSwap {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, self.wires());
    }
}

impl UsedWires<2> for RotationSwap {
    fn wires(&self) -> [usize; 2] {
        [self.wire_1, self.wire_0]
    }
}

impl Parameterized<2> for RotationSwap {
    fn parameterized(&self) -> fn(&RotationSwap, f64) -> Gate<2> {
        fn create_parameterized(g: &RotationSwap, theta: f64) -> Gate<2> {
            RotationSwap::new(g.wire_0, g.wire_1, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}

#[cfg(test)]
mod test {
    use core::f64::consts::PI;
    use float_cmp::assert_approx_eq;
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::rotation::rotation_swap::RotationSwap;
    use crate::quantum::operator::simple::swap::Swap;
    use crate::quantum::operator::simple::swap_root::SwapRoot;
    use crate::quantum::operator::traits::ToGate;

    #[test]
    fn test_full() {
        let gate_rswap = RotationSwap::new(0, 0, PI).to_gate();
        let gate_swap = Swap::new(0, 0).to_gate();
        assert_approx_eq!(Gate<2>, gate_rswap, gate_swap, epsilon = 0.000001);
    }

    #[test]
    fn test_half() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 2.0).to_gate();
        let gate_swap = Swap::new(0, 0).to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate_rswap * gate_rswap,
            gate_swap,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_third() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 3.0).to_gate();
        let gate_swap = Swap::new(0, 0).to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate_rswap * gate_rswap * gate_rswap,
            gate_swap,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_quarter() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 4.0).to_gate();
        let gate_swap = Swap::new(0, 0).to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate_rswap * gate_rswap * gate_rswap * gate_rswap,
            gate_swap,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_root() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 2.0).to_gate();
        let gate_swap_root = SwapRoot::new(0, 0).to_gate();
        assert_approx_eq!(Gate<2>, gate_rswap, gate_swap_root, epsilon = 0.000001);
    }

    #[test]
    fn test_root_half() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 4.0).to_gate();
        let gate_swap_root = SwapRoot::new(0, 0).to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate_rswap * gate_rswap,
            gate_swap_root,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_root_third() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 6.0).to_gate();
        let gate_swap_root = SwapRoot::new(0, 0).to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate_rswap * gate_rswap * gate_rswap,
            gate_swap_root,
            epsilon = 0.000001
        );
    }

    #[test]
    fn test_root_quarter() {
        let gate_rswap = RotationSwap::new(0, 0, PI / 8.0).to_gate();
        let gate_swap_root = SwapRoot::new(0, 0).to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate_rswap * gate_rswap * gate_rswap * gate_rswap,
            gate_swap_root,
            epsilon = 0.000001
        );
    }
}
