use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::util::trig::{const_cos, const_sin};
use crate::impl_rotation;

const fn rotation_z(theta: f64) -> ConstSizedMatrix<1, Complex> {
    ConstSizedMatrix::new([
        [Complex::new(const_cos(theta / 2.0), const_sin(-theta / 2.0)), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(const_cos(theta / 2.0), const_sin(theta / 2.0))],
    ])
}

impl_rotation!(rotation_z, Complex, RotationZ, 1, "");

#[cfg(test)]
mod test {
    use super::*;
    use core::f64::consts::{PI, FRAC_PI_2, FRAC_1_SQRT_2, SQRT_2};
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
        assert_eq!(*RotationZ::new(0.0, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]))
    }

    #[test]
    fn test_z() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(HADAMARD.into(), [0], None).apply(ket, &mut register);
        let ket = RotationZ::new(PI, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, -FRAC_1_SQRT_2), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, FRAC_1_SQRT_2), epsilon = 0.00000003);

        let ket = RotationZ::new(-PI, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_half_z() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(HADAMARD.into(), [0], None).apply(ket, &mut register);
        let ket = RotationZ::new(FRAC_PI_2, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.5, -0.5), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.5, 0.5), epsilon = 0.00000003);

        let ket = RotationZ::new(-FRAC_PI_2, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_half_z_half_z() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedUnitaryOperator::new(HADAMARD.into(), [0], None).apply(ket, &mut register);
        let ket = RotationZ::new(FRAC_PI_2, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.5, -0.5), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.5, 0.5), epsilon = 0.00000003);

        let ket = RotationZ::new(FRAC_PI_2, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(0.0, -FRAC_1_SQRT_2), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, FRAC_1_SQRT_2), epsilon = 0.00000003);

        let ket = RotationZ::new(-PI, 0).apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(FRAC_1_SQRT_2, 0.0), epsilon = 0.00000003);
    }
}
