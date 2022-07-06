use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::util::trig::{const_cos, const_sin};
use crate::impl_rotation;

const fn rotation_pauli_y(theta: f64) -> ConstSizedMatrix<1, Complex> {
    Complex::new(const_cos(theta / 2.0), const_sin(theta / 2.0)) * ConstSizedMatrix::new([
        [Complex::new(const_cos(theta / 2.0), 0.0), Complex::new(-const_sin(theta / 2.0), 0.0)],
        [Complex::new(const_sin(theta / 2.0), 0.0), Complex::new(const_cos(theta / 2.0), 0.0)],
    ])
}

impl_rotation!(rotation_pauli_y, Complex, RotationPauliY, 1, "");

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
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationPauliY::new(0.0, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]))
    }

    #[test]
    fn test_y() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationPauliY::new(PI, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_ry() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = RotationPauliY::new(FRAC_PI_2, 0).apply(ket, &mut register);
        let ket = RotationPauliY::new(FRAC_PI_2, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 1.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_rry() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = RotationPauliY::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationPauliY::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationPauliY::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationPauliY::new(FRAC_PI_4, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 1.0), epsilon = 0.00000003);
    }
}
