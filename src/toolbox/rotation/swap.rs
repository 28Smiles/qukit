use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::util::trig::{const_cos, const_sin};
use crate::impl_rotation;

const S: ConstSizedMatrix<2, f64> = ConstSizedMatrix::new([
    [ 0.0, 0.0, 0.0, 1.0 ],
    [ -1.0, 0.0, 1.0, 0.0 ],
    [ 1.0, 0.0, 1.0, 0.0 ],
    [ 0.0, 1.0, 0.0, 0.0 ],
]);
const S_INV: ConstSizedMatrix<2, f64> = ConstSizedMatrix::new([
    [ 0.0, -0.5, 0.5, 0.0 ],
    [ 0.0, 0.0, 0.0, 1.0 ],
    [ 0.0, 0.5, 0.5, 0.0 ],
    [ 1.0, 0.0, 0.0, 0.0 ],
]);

const fn rotation_swap(theta: f64) -> ConstSizedMatrix<2, Complex> {
    S * ConstSizedMatrix::new([
        [Complex::new(const_cos(theta), const_sin(theta)), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]) * S_INV
}

impl_rotation!(rotation_swap, Complex, RotationSwap, 2, "");

#[cfg(test)]
mod test {
    use super::*;
    use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4, FRAC_1_SQRT_2, SQRT_2};
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
    fn test_i() {
        assert_approx_eq!(ConstSizedMatrix<2, Complex>, *RotationSwap::new(0.0, 0, 1).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]))
    }

    #[test]
    fn test_x() {
        assert_approx_eq!(ConstSizedMatrix<2, Complex>, *RotationSwap::new(PI, 0, 1).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_rx() {
        assert_approx_eq!(ConstSizedMatrix<2, Complex>, *RotationSwap::new(FRAC_PI_2, 0, 1).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.5, 0.5), Complex::new(0.5, -0.5), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.5, -0.5), Complex::new(0.5, 0.5), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_rrx() {
        let ket = Ket::new(2).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(HADAMARD.into(), [0], None).apply(ket, &mut register);
        let ket = RotationSwap::new(FRAC_PI_4, 0, 1).apply(ket, &mut register);
        let ket = RotationSwap::new(FRAC_PI_4, 0, 1).apply(ket, &mut register);
        let ket = RotationSwap::new(FRAC_PI_4, 0, 1).apply(ket, &mut register);
        let ket = RotationSwap::new(FRAC_PI_4, 0, 1).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(2).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(3).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
    }
}
