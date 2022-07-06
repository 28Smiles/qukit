use core::ops::Deref;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator;
use crate::util::const_iter::ConstIter;
use crate::util::one::One;
use crate::util::s_cow::SCow;

#[derive(Copy, Clone, PartialEq)]
pub(crate)struct Controlled<const SIZE: usize, T: Sized + Copy + 'static>(ConstSizedUnitaryOperator<SIZE, T>)
    where
        [(); 0x1 << SIZE]:,;

impl<const SIZE: usize, T: Sized + Copy + Default + One> Controlled<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    pub(crate)const fn new<CT: Sized + Copy + ~const Default + ~const One, O: ~const Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, CT>>>(wire: usize, inner: O) -> Controlled<SIZE, CT> {
        let mut matrix = [[CT::default(); 0x1 << SIZE]; 0x1 << SIZE];
        for i in ConstIter(0, 0x1 << (SIZE - 1)) {
            matrix[i][i] = CT::one();
        }
        let inner_unitary = inner.into();
        let inner_unitary_matrix = inner_unitary.matrix();
        let inner_matrix = inner_unitary_matrix.matrix();
        for i in ConstIter(0, 0x1 << (SIZE - 1)) {
            for j in ConstIter(0, 0x1 << (SIZE - 1)) {
                matrix[i + (0x1 << (SIZE - 1))][j + (0x1 << (SIZE - 1))] = inner_matrix[i][j];
            }
        }

        let mut wires = [0; SIZE];
        wires[0] = wire;
        let inner_wires = inner_unitary.wires();
        for i in ConstIter(1, SIZE) {
            wires[i] = inner_wires[i - 1];
        }

        Controlled(ConstSizedUnitaryOperator::new(
            SCow::Owned(ConstSizedMatrix::new(matrix)),
            wires,
            None,
        ))
    }

    pub(crate)fn operator(self) -> ConstSizedUnitaryOperator<SIZE, T> {
        self.0
    }
}

impl<const SIZE: usize, T: Sized + Copy> const Deref for Controlled<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    type Target = ConstSizedUnitaryOperator<SIZE, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const SIZE: usize, T: Sized + Copy> const Into<ConstSizedUnitaryOperator<SIZE, T>> for Controlled<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    fn into(self) -> ConstSizedUnitaryOperator<SIZE, T> {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;
    use crate::complex::Complex;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    const PAULI_X: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]);

    #[test]
    fn test_controlled_off() {
        let ket = Ket::new(2).unwrap();
        let mut register = Register::new(0);
        let pauli_x = ConstSizedUnitaryOperator::new(PAULI_X.into(), [1], None);
        let controlled: Controlled<2, Complex> = Controlled::<2, Complex>::new(0, pauli_x);
        let ket = controlled.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.0, epsilon = 0.00000003);
    }

    #[test]
    fn test_controlled_on() {
        let ket = Ket::new(2).unwrap();
        let mut register = Register::new(0);
        let pauli_x = ConstSizedUnitaryOperator::new(PAULI_X.into(), [1], None);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let controlled: Controlled<2, Complex> = Controlled::<2, Complex>::new(0, pauli_x);
        let ket = controlled.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 1.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 1.0, epsilon = 0.00000003);
    }

    #[test]
    fn test_toffoli_off() {
        let ket = Ket::new(3).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let controlled: Controlled<3, Complex> = Controlled::<3, Complex>::new(0, Controlled::<2, Complex>::new(1, ConstSizedUnitaryOperator::new(PAULI_X.into(), [2], None)));
        let ket = controlled.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 1.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(2), 0.0, epsilon = 0.00000003);
    }

    #[test]
    fn test_toffoli_on() {
        let ket = Ket::new(3).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [1], None).apply(ket, &mut register);
        let controlled: Controlled<3, Complex> = Controlled::<3, Complex>::new(0, Controlled::<2, Complex>::new(1, ConstSizedUnitaryOperator::new(PAULI_X.into(), [2], None)));
        let ket = controlled.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 1.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 1.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(2), 1.0, epsilon = 0.00000003);
    }
}
