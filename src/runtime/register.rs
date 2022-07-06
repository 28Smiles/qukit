use alloc::vec::Vec;
use alloc::vec;
use core::fmt::{Display, Formatter};

#[derive(Clone)]
pub(crate) struct Register {
    bits: Vec<bool>
}

impl Register {
    pub(crate) fn new(size: usize) -> Register {
        Register {
            bits: vec![false; size],
        }
    }

    pub(crate) fn set(&mut self, index: usize, value: bool) {
        self.bits.insert(index, value)
    }

    pub(crate) fn get(&mut self, index: usize) -> Option<&bool> {
        self.bits.get(index)
    }

    pub(crate) fn bits(&self) -> &Vec<bool> {
        &self.bits
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for value in self.bits.iter().rev() {
            if *value {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
        }

        Ok(())
    }
}
