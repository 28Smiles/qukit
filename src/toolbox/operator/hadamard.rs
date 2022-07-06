use core::f64::consts::SQRT_2;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::impl_operator;

const HADAMARD: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
    [1.0 / SQRT_2, 1.0 / SQRT_2],
    [1.0 / SQRT_2, -1.0 / SQRT_2],
]);

impl_operator!(HADAMARD, f64, Complex, Hadamard, crate::toolbox::rotation::hadamard::RotationHadamard, 1, 1.0, "");

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    #[test]
    fn test_hadamard() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let hadamard = Hadamard::new(0);
        let ket = hadamard.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.5, epsilon = 0.00000003);
    }

    #[test]
    fn test_hadamard_hadamard() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let hadamard = Hadamard::new(0);
        let ket = hadamard.apply(ket, &mut register);
        let ket = hadamard.apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.0, epsilon = 0.00000003);
    }
}
