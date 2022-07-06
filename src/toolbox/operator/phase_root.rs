use core::f64::consts::SQRT_2;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::{ConstSizedMatrix, Transpose};
use crate::impl_operator;

const PHASE_ROOT: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    [Complex::new(0.0, 0.0), Complex::new(1.0 / SQRT_2, 1.0 / SQRT_2)],
]);
const PHASE_ROOT_DAGGER: ConstSizedMatrix<1, Complex> = PHASE_ROOT.transpose().conjugate();

impl_operator!(PHASE_ROOT, Complex, Complex, PhaseRoot, crate::toolbox::rotation::pauli_z::RotationPauliZ, 1, 4.0, "");
impl_operator!(PHASE_ROOT_DAGGER, Complex, Complex, PhaseRootDagger, crate::toolbox::rotation::pauli_z::RotationPauliZ, 1, -4.0, "");

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;
    use crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    const PAULI_X: ConstSizedMatrix<1, Complex> = ConstSizedMatrix::new([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]);

    #[test]
    fn test_phase() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(1.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_x_phase() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 1.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_x_phase_dagger() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, -1.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_x_phase_phase() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(-1.0, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_x_phase_dagger_phase_dagger() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(-1.0, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_x_phase_phase_dagger() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(PAULI_X.into(), [0], None).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        let ket = PhaseRoot::new(0).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        let ket = PhaseRootDagger::new(0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(1.0, 0.0), epsilon = 0.00000003);
    }
}
