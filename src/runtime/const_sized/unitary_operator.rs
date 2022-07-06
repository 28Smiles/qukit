use core::fmt::{Display, Formatter};
use core::ops::Mul;
use core::ops::Deref;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::runtime::ket::Ket;
use crate::runtime::matrix::Matrix;
use crate::runtime::register::Register;
use crate::runtime::unitary::UnitaryOperator;
use crate::util::s_cow::SCow;

#[derive(Copy, Clone, PartialEq)]
pub(crate)struct ConstSizedUnitaryOperator<const SIZE: usize, T: Sized + Copy + 'static>
    where
        [(); 0x1 << SIZE]:,
{
    matrix: SCow<ConstSizedMatrix<SIZE, T>>,
    wires: [usize; SIZE],
    classical_control: Option<usize>,
}

impl<const SIZE: usize, T: Sized + Copy> ConstSizedUnitaryOperator<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    /// Create a new instance of the [ConstSizedUnitaryOperator](ConstSizedUnitaryOperator).
    pub(crate) const fn new(
        matrix: SCow<ConstSizedMatrix<SIZE, T>>,
        wires: [usize; SIZE],
        classical_control: Option<usize>,
    ) -> ConstSizedUnitaryOperator<SIZE, T> {
        ConstSizedUnitaryOperator {
            matrix,
            wires,
            classical_control,
        }
    }

    pub(crate)const fn wires(&self) -> &[usize; SIZE] {
        &self.wires
    }

    pub(crate)const fn matrix(&self) -> &ConstSizedMatrix<SIZE, T> {
        self.matrix.deref()
    }

    pub(crate)const fn classical_control(&self) -> Option<usize> {
        self.classical_control
    }
}

impl<
    const SIZE: usize,
    T: Mul<Complex, Output = Complex> + Copy + Sync + Send + Sized
> UnitaryOperator for ConstSizedUnitaryOperator<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket {
        if let Some(classical_control) = &self.classical_control {
            if *register.get(*classical_control).unwrap() {
                self.matrix.apply(ket, &self.wires)
            } else {
                ket
            }
        } else {
            self.matrix.apply(ket, &self.wires)
        }
    }
}

impl<const SIZE: usize, T: Display + Copy> Display for ConstSizedUnitaryOperator<SIZE, T>
    where
        [(); 0x1 << SIZE]:,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.matrix.fmt(f)
    }
}
