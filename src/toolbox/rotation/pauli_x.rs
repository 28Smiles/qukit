use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::util::trig::{const_cos, const_sin};
use crate::impl_rotation;

const fn rotation_pauli_x(theta: f64) -> ConstSizedMatrix<1, Complex> {
    Complex::new(const_cos(theta / 2.0), const_sin(theta / 2.0)) * ConstSizedMatrix::new([
        [Complex::new(const_cos(theta / 2.0), 0.0), Complex::new(0.0, -const_sin(theta / 2.0))],
        [Complex::new(0.0, -const_sin(theta / 2.0)), Complex::new(const_cos(theta / 2.0), 0.0)],
    ])
}

impl_rotation!(rotation_pauli_x, Complex, RotationPauliX, 1, "");

#[cfg(test)]
mod test {
    use super::*;
    use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};
    use float_cmp::assert_approx_eq;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    #[test]
    fn test_i() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationPauliX::new(0.0, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]))
    }

    #[test]
    fn test_x() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationPauliX::new(PI, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_rx() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationPauliX::new(FRAC_PI_2, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)],
            [Complex::new(0.5, -0.5), Complex::new(0.5, 0.5)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_rrx() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = RotationPauliX::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationPauliX::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationPauliX::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationPauliX::new(FRAC_PI_4, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(1.0, 0.0), epsilon = 0.00000003);
    }
}
