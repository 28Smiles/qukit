use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::impl_operator;

const PAULI_Z: ConstSizedMatrix<1, f64> = ConstSizedMatrix::new([
    [1.0, 0.0],
    [0.0, -1.0],
]);

impl_operator!(PAULI_Z, f64, Complex, PauliZ, crate::toolbox::rotation::pauli_z::RotationPauliZ, 1, 1.0, "");

#[cfg(test)]
mod test {
    use std::f64::consts::SQRT_2;
    use super::*;
    use float_cmp::assert_approx_eq;
    use crate::runtime::ket::Ket;
    use crate::runtime::unitary::UnitaryOperator;
    use crate::complex::Complex;
    use crate::runtime::matrix::Matrix;
    use crate::runtime::register::Register;

    #[test]
    fn test_pauli_z() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let pauli_z = PauliZ::new(0);
        let ket = pauli_z.apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(1.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_pauli_zz() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let pauli_z = PauliZ::new(0);
        let ket = pauli_z.apply(ket, &mut register);
        let ket = pauli_z.apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(1.0, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(0.0, 0.0), epsilon = 0.00000003);
    }

    #[test]
    fn test_pauli_xz() {
        let ket = Ket::new(1).unwrap();
        let mut register = Register::new(0);
        let ket = ConstSizedMatrix::new([
            [Complex::new(1.0 / SQRT_2, 0.0), Complex::new(1.0 / SQRT_2, 0.0)],
            [Complex::new(1.0 / SQRT_2, 0.0), Complex::new(-1.0 / SQRT_2, 0.0)],
        ]).apply(ket, &[ 0 ]);
        let pauli_z = PauliZ::new(0);
        let ket = pauli_z.apply(ket, &mut register);
        assert_approx_eq!(Complex, *ket.state().get(0).unwrap(), Complex::new(1.0 / SQRT_2, 0.0), epsilon = 0.00000003);
        assert_approx_eq!(Complex, *ket.state().get(1).unwrap(), Complex::new(-1.0 / SQRT_2, 0.0), epsilon = 0.00000003);
    }
}
