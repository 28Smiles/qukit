use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::impl_operator;

const PAULI_X_ROOT: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
    [Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)],
    [Complex::new(0.5, -0.5), Complex::new(0.5, 0.5)],
]);

impl_operator!(PAULI_X_ROOT, Complex, Complex, PauliXRoot, crate::toolbox::rotation::pauli_x::RotationPauliX, 1, 2.0, "");

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    #[test]
    fn test_pauli_x() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let pauli_x = PauliXRoot::new(0);
        let ket = pauli_x.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.5, epsilon = 0.00000003);
    }

    #[test]
    fn test_pauli_xx() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let pauli_x = PauliXRoot::new(0);
        let ket = pauli_x.apply(ket, &mut register);
        let ket = pauli_x.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 1.0, epsilon = 0.00000003);
    }

    #[test]
    fn test_pauli_xxxx() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let pauli_x = PauliXRoot::new(0);
        let ket = pauli_x.apply(ket, &mut register);
        let ket = pauli_x.apply(ket, &mut register);
        let ket = pauli_x.apply(ket, &mut register);
        let ket = pauli_x.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.0, epsilon = 0.00000003);
    }
}
