use crate::runtime::ket::Ket;

pub(crate) trait Matrix<const SIZE: usize> {
    fn apply(&self, ket: Ket, wires: &[usize; SIZE]) -> Ket;
}
