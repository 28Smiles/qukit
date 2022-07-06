use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::impl_operator;

const SWAP: ConstSizedMatrix<2, f64> = ConstSizedMatrix::new([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

impl_operator!(SWAP, f64, Complex, Swap, crate::toolbox::rotation::swap::RotationSwap, 2, 1.0, "");

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::SQRT_2;
    use float_cmp::assert_approx_eq;
    use crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    const HADAMARD: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
        [Complex::new(1.0 / SQRT_2, 0.0), Complex::new(1.0 / SQRT_2, 0.0)],
        [Complex::new(1.0 / SQRT_2, 0.0), Complex::new(-1.0 / SQRT_2, 0.0)],
    ]);

    #[test]
    fn test_h_swap() {
        let ket = Ket::new(2).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(HADAMARD.into(), [0], None).apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.5, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.0, epsilon = 0.00000003);
        let ket = Swap::new(0, 1).apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.5, epsilon = 0.00000003);
        let ket = Swap::new(0, 1).apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.5, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.0, epsilon = 0.00000003);
    }

    #[test]
    fn test_swap() {
        let ket = Ket::new(2).unwrap();
        let mut register = Register::new(0);
        assert_approx_eq!(f64, ket.probability(0), 0.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.0, epsilon = 0.00000003);
        let ket = Swap::new(0, 1).apply(ket, &mut register);
        assert_approx_eq!(f64, ket.probability(0), 0.0, epsilon = 0.00000003);
        assert_approx_eq!(f64, ket.probability(1), 0.0, epsilon = 0.00000003);
    }
}
