use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::matrix::const_sized::Gate;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct Controlled<
    const SIZE: usize,
    T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized,
> where
    [(); 0x1 << SIZE]:,
    [(); 0x1 << { SIZE - 1 }]:,
{
    wire: u32,
    transformation: T,
}

impl<const SIZE: usize, T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized>
    Controlled<SIZE, T>
where
    [(); 0x1 << SIZE]:,
    [(); 0x1 << { SIZE - 1 }]:,
{
    pub fn new(wire: u32, transformation: T) -> Controlled<SIZE, T> {
        Controlled {
            wire,
            transformation,
        }
    }
}

fn add_control<const SIZE: usize>(inner_gate: Gate<{ SIZE - 1 }>) -> Gate<SIZE>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    let mut m: [[Complex; 0x1 << SIZE]; 0x1 << SIZE] =
        [[Complex::zero(); 0x1 << SIZE]; 0x1 << SIZE];
    for i in 0..0x1 << (SIZE - 1) {
        m[i][i] = Complex::new(1.0, 0.0);
    }
    let im = inner_gate.matrix();
    for x in 0..0x1 << (SIZE - 1) {
        for y in 0..0x1 << (SIZE - 1) {
            let mx = x + (0x1 << (SIZE - 1));
            let my = y + (0x1 << (SIZE - 1));
            m[mx][my] = im[x][y]
        }
    }

    Gate::new(m)
}

impl<const SIZE: usize, T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized> ToGate<SIZE>
    for Controlled<SIZE, T>
where
    [(); 0x1 << SIZE]:,
    [(); 0x1 << { SIZE - 1 }]:,
{
    fn to_gate(&self) -> Gate<SIZE> {
        let inner_gate: Gate<{ SIZE - 1 }> = self.transformation.to_gate();

        add_control(inner_gate)
    }
}

impl<const SIZE: usize, T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized> UsedWires<SIZE>
    for Controlled<SIZE, T>
where
    [(); 0x1 << SIZE]:,
    [(); 0x1 << { SIZE - 1 }]:,
{
    fn wires(&self) -> [u32; SIZE] {
        let mut w: [u32; SIZE] = [0; SIZE];
        w[SIZE - 1] = self.wire;
        let iw: [u32; SIZE - 1] = self.transformation.wires();
        for i in 0..SIZE - 1 {
            w[i] = iw[i];
        }

        w
    }
}

impl<const SIZE: usize, T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized> ApplyGate<SIZE>
for Controlled<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, self.wires());
    }
}

impl<const SIZE: usize, T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized> Parameterized<SIZE>
for Controlled<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    fn parameterized(&self) -> fn(&Controlled<SIZE, T>, f64) -> Gate<SIZE> {
        fn create_parameterized<const SIZE: usize, T: UsedWires<{ SIZE - 1 }> + ToGate<{ SIZE - 1 }> + Parameterized<{ SIZE - 1 }> + Copy + Sized>(g: &Controlled<SIZE, T>, theta: f64) -> Gate<SIZE>
            where
                [(); 0x1 << SIZE]:,
                [(); 0x1 << { SIZE - 1 }]:, {
            add_control(g.transformation.parameterized()(&g.transformation, theta))
        }

        create_parameterized
    }
}

#[cfg(test)]
mod test {
    use crate::complex::Complex;
    use float_cmp::assert_approx_eq;
    use crate::quantum::gate::matrix::const_sized::Gate;
    use crate::quantum::operator::simple::controlled::Controlled;
    use crate::quantum::operator::simple::pauli_x::PauliX;
    use crate::quantum::operator::simple::pauli_y::PauliY;
    use crate::quantum::operator::simple::pauli_z::PauliZ;
    use crate::quantum::operator::traits::ToGate;

    #[test]
    fn test_cnot() {
        let cnot: Controlled<2, _> = Controlled::new(0, PauliX::new(1));
        let gate = cnot.to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate,
            Gate::new([
                [
                    Complex::new(1.0, 0.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::new(1.0, 0.0),
                    Complex::zero(),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::new(1.0, 0.0)
                ],
                [
                    Complex::zero(),
                    Complex::zero(),
                    Complex::new(1.0, 0.0),
                    Complex::zero()
                ]
            ])
        )
    }

    #[test]
    fn test_cy() {
        let cnot: Controlled<2, _> = Controlled::new(0, PauliY::new(1));
        let gate = cnot.to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate,
            Gate::new([
                [
                    Complex::new(1.0, 0.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::new(1.0, 0.0),
                    Complex::zero(),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::new(0.0, 1.0)
                ],
                [
                    Complex::zero(),
                    Complex::zero(),
                    Complex::new(0.0, -1.0),
                    Complex::zero()
                ]
            ])
        )
    }

    #[test]
    fn test_cz() {
        let cnot: Controlled<2, _> = Controlled::new(0, PauliZ::new(1));
        let gate = cnot.to_gate();
        assert_approx_eq!(
            Gate<2>,
            gate,
            Gate::new([
                [
                    Complex::new(1.0, 0.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::new(1.0, 0.0),
                    Complex::zero(),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::zero(),
                    Complex::new(1.0, 0.0),
                    Complex::zero()
                ],
                [
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::new(-1.0, 0.0)
                ]
            ])
        )
    }
}
