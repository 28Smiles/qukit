use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::matrix::const_sized::Gate;

pub trait ToGate<const SIZE: usize>
    where
        [(); 0x1 << SIZE]:,
{
    fn to_gate(&self) -> Gate<SIZE>;
}

pub trait ApplyGate<const SIZE: usize>
    where
        [(); 0x1 << SIZE]:, {
    fn apply(&self, computer: &mut QuantumComputer);
}

pub trait UsedWires<const SIZE: usize>
    where
        [(); 0x1 << SIZE]:,
{
    fn wires(&self) -> [u32; SIZE];
}

pub trait Parameterized<const SIZE: usize>
    where
        [(); 0x1 << SIZE]:,
{
    fn parameterized(&self) -> fn(&Self, f64) -> Gate<SIZE>;
}

pub trait ApplyGateParameterized<const SIZE: usize>: Parameterized<SIZE> + UsedWires<SIZE>
    where
        [(); 0x1 << SIZE]:, {
    fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer);
}

impl <const SIZE: usize, T: Parameterized<SIZE> + UsedWires<SIZE>>ApplyGateParameterized<SIZE> for T
    where
        [(); 0x1 << SIZE]:, {
    fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer) {
        self.parameterized()(self, theta).apply(computer, self.wires());
    }
}
