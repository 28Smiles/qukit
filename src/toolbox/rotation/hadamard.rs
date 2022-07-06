use core::f64::consts::SQRT_2;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::util::trig::{const_cos, const_sin};
use crate::impl_rotation;

const S: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
    [1.0 - SQRT_2, 1.0 + SQRT_2],
    [1.0, 1.0],
]);
const S_INV: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
    [-1.0 / (2.0 * SQRT_2), 0.25 * (2.0 + SQRT_2) ],
    [1.0 / (2.0 * SQRT_2), 0.25 * (2.0 - SQRT_2) ],
]);

const fn rotation_hadamard(theta: f64) -> ConstSizedMatrix<1, Complex> {
    S * ConstSizedMatrix::new([
        [Complex::new(const_cos(theta), const_sin(theta)), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]) * S_INV
}

impl_rotation!(rotation_hadamard, Complex, RotationHadamard, 1, "");

#[cfg(test)]
mod test {
    use super::*;
    use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4, FRAC_1_SQRT_2};
    use float_cmp::assert_approx_eq;
    use crate::runtime::ket::Ket;
    use crate::runtime::register::Register;
    use crate::runtime::unitary::UnitaryOperator;

    #[test]
    fn test_i() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationHadamard::new(0.0, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]))
    }

    #[test]
    fn test_h() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationHadamard::new(PI, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(FRAC_1_SQRT_2, 0.0), Complex::new(FRAC_1_SQRT_2, 0.0)],
            [Complex::new(FRAC_1_SQRT_2, 0.0), Complex::new(-FRAC_1_SQRT_2, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_rrh() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = RotationHadamard::new(FRAC_PI_2, 0).apply(ket, &mut register);
        let ket = RotationHadamard::new(FRAC_PI_2, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_rrrrh() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = RotationHadamard::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationHadamard::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationHadamard::new(FRAC_PI_4, 0).apply(ket, &mut register);
        let ket = RotationHadamard::new(FRAC_PI_4, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
    }
}
