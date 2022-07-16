use core::f64::consts::PI;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::util::trig::{const_cos, const_sin};
use crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator;
use crate::toolbox::parameterized::Parameterized;
use crate::util::s_cow::SCow;

const fn rotation_u(theta: f64, phi: f64, lambda: f64) -> ConstSizedMatrix<1, Complex> {
    ConstSizedMatrix::new([
        [
            Complex::new(const_cos(theta / 2.0), 0.0),
            Complex::new(-const_cos(lambda) * const_sin(theta / 2.0), -const_sin(lambda) * const_sin(theta / 2.0))
        ],
        [
            Complex::new(const_cos(phi) * const_sin(theta / 2.0), const_sin(phi) * const_sin(theta / 2.0)),
            Complex::new(const_cos(phi + lambda) * const_cos(theta / 2.0), const_sin(phi + lambda) * const_cos(theta / 2.0))
        ],
    ])
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct RotationU(f64, f64, f64, ConstSizedUnitaryOperator<1, Complex>);

impl RotationU {
    pub(crate) const fn new(theta: f64, phi: f64, lambda: f64, wire: usize) -> RotationU {
        RotationU(theta, phi, lambda, ConstSizedUnitaryOperator::new(
            SCow::Owned(rotation_u(theta, phi, lambda)),
            [wire],
            None,
        ))
    }

    pub(crate) const fn new_classically_controlled(theta: f64, phi: f64, lambda: f64, wire: usize, classical_control: usize) -> RotationU {
        RotationU(theta, phi, lambda, ConstSizedUnitaryOperator::new(
            SCow::Owned(rotation_u(theta, phi, lambda)),
            [wire],
            Some(classical_control),
        ))
    }

    pub(crate) fn operator(self) -> ConstSizedUnitaryOperator<1, Complex> {
        self.3
    }
}

impl const Parameterized<ConstSizedUnitaryOperator<1, Complex>, RotationU> for RotationU {
    fn parameterized(&self, theta: f64) -> RotationU {
        if let Some(classical_control) = self.classical_control() {
            RotationU::new_classically_controlled(self.0 / PI * theta, self.1 / PI * theta, self.2 / PI * theta, self.wires()[0], classical_control)
        } else {
            RotationU::new(self.0 / PI * theta, self.1 / PI * theta, self.2 / PI * theta, self.wires()[0])
        }
    }
}

impl const core::ops::Deref for RotationU {
    type Target = ConstSizedUnitaryOperator<1, Complex>;

    fn deref(&self) -> &Self::Target {
        &self.3
    }
}

impl const Into<ConstSizedUnitaryOperator<1, Complex>> for RotationU {
    fn into(self) -> ConstSizedUnitaryOperator<1, Complex> {
        self.3
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::f64::consts::{PI, SQRT_2};
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_i() {
        assert_eq!(*RotationU::new(0.0, 0.0, 0.0, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]))
    }

    #[test]
    fn test_x() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationU::new(PI, 0.0, PI, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_h() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationU::new(PI / 2.0, 0.0, PI, 0).matrix(), ConstSizedMatrix::new([
        [Complex::new(1.0 / SQRT_2, 0.0), Complex::new(1.0 / SQRT_2, 0.0)],
        [Complex::new(1.0 / SQRT_2, 0.0), Complex::new(-1.0 / SQRT_2, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_y() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationU::new(PI, PI / 2.0, PI / 2.0, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
        ]), epsilon = 0.00000003)
    }

    #[test]
    fn test_z() {
        assert_approx_eq!(ConstSizedMatrix<1, Complex>, *RotationU::new(0.0, 0.0, PI, 0).matrix(), ConstSizedMatrix::new([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
        ]), epsilon = 0.00000003)
    }
}
