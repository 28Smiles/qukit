use alloc::vec::Vec;
use alloc::format;
use crate::error::{QuantumError, Result};
use crate::runtime::dynamic_sized::matrix::DynamicSizedMatrix;
use crate::runtime::unitary::UnitaryOperator;
use crate::runtime::ket::Ket;
use crate::runtime::register::Register;

#[derive(Clone, Debug, PartialEq)]
pub(crate)struct DynamicSizedUnitaryOperator {
    matrix: DynamicSizedMatrix,
    wires: Vec<usize>,
    classical_control: Option<usize>,
}

impl DynamicSizedUnitaryOperator {
    /// Create a new instance of the [DynamicSizedUnitaryOperator](DynamicSizedUnitaryOperator).
    pub(crate) fn new(
        matrix: DynamicSizedMatrix,
        wires: Vec<usize>,
        classical_control: Option<usize>,
    ) -> Result<DynamicSizedUnitaryOperator> {
        if matrix.size() != wires.len() {
            return Err(QuantumError(format!(
                    "Expecting matrix to be applied to {} qbits, but where {} qbits",
                    matrix.size(),
                    wires.len()
            )))
        }

        Ok(DynamicSizedUnitaryOperator {
            matrix,
            wires,
            classical_control,
        })
    }

    pub(crate) fn wires(&self) -> &[usize] {
        self.wires.as_slice()
    }

    pub(crate) fn matrix(&self) -> &DynamicSizedMatrix {
        &self.matrix
    }

    pub(crate) fn classical_control(&self) -> Option<usize> {
        self.classical_control
    }
}

impl UnitaryOperator for DynamicSizedUnitaryOperator {
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket {
        if let Some(classical_control) = &self.classical_control {
            if *register.get(*classical_control).unwrap() {
                self.matrix.apply(ket, self.wires.as_slice())
            } else {
                ket.clone()
            }
        } else {
            self.matrix.apply(ket, self.wires.as_slice())
        }
    }
}
