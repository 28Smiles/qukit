use libm::sqrt;
use crate::complex::Complex;
use crate::runtime::const_sized::matrix::ConstSizedMatrix;
use crate::runtime::ket::Ket;
use crate::runtime::matrix::Matrix;
use crate::runtime::register::Register;
use crate::runtime::unitary::UnitaryOperator;

#[derive(Copy, Clone, PartialEq)]
pub(crate)struct Reset {
    wire: usize,
    state: bool,
}

impl Reset {
    pub(crate) fn new(wire: usize, state: bool) -> Reset {
        Reset {
            wire,
            state,
        }
    }
}

impl UnitaryOperator for Reset {
    fn apply(&self, ket: Ket, _: &mut Register) -> Ket {
        let probability = ket.probability(self.wire);

        if self.state {
            ConstSizedMatrix::new([
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(sqrt(1.0 / probability), 0.0)],
            ])
        } else {
            ConstSizedMatrix::new([
                [Complex::new(sqrt(1.0 / probability), 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            ])
        }.apply(ket, &[ self.wire ])
    }
}
