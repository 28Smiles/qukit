use crate::complex::Complex;
use alloc::format;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::ops::{Deref, DerefMut};
use alloc::vec;
use bit_array::BitArray;

#[cfg(target_pointer_width = "32")]
#[derive(Eq, PartialEq, Clone)]
pub(crate) struct IndexType(BitArray<usize, typenum::U32>);
#[cfg(target_pointer_width = "64")]
#[derive(Eq, PartialEq, Clone)]
pub(crate) struct IndexType(BitArray<usize, typenum::U64>);

impl Into<usize> for IndexType {
    #[inline(always)]
    fn into(self) -> usize {
        self.0.storage()[0]
    }
}

impl From<usize> for IndexType {
    #[inline(always)]
    fn from(value: usize) -> Self {
        let mut idx = IndexType(BitArray::new());
        unsafe { idx.0.storage_mut()[0] = value };

        return idx
    }
}

impl Deref for IndexType {
    #[cfg(target_pointer_width = "32")]
    type Target = BitArray<usize, typenum::U32>;
    #[cfg(target_pointer_width = "64")]
    type Target = BitArray<usize, typenum::U64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IndexType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Ket {
    pub(crate) size: usize,
    pub(crate) vec: Vec<Complex>,
}

impl Ket {
    pub fn new(size: usize) -> Ket {
        let mut k = Ket::new_zero(size);
        #[cfg(all(not(test), not(feature = "safe")))]
        {
            *unsafe { k.vec.get_unchecked_mut(0) } = Complex::new(1.0, 0.0);
        }
        #[cfg(any(test, feature = "safe"))]
        {
            *{ k.vec.get_mut(0).unwrap() } = Complex::new(1.0, 0.0);
        }

        k
    }

    pub(crate) fn new_zero(size: usize) -> Ket {
        assert!(size > 0);
        let vec = vec![Complex::zero(); 0x1 << size];

        Ket { size, vec }
    }

    pub fn amplitudes(&self) -> Vec<f64> {
        return self
            .vec
            .iter()
            .map(|c| {
                let a = c.abs();
                a * a
            })
            .collect();
    }
}

impl Display for Ket {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for (i, a) in self.amplitudes().iter().enumerate() {
            let s = format!("{:#0width$b}", i, width = self.size as usize + 2);
            let (_, s) = s.split_at(2);
            writeln!(f, "|{}> = {:05.2}%", s, a * 100.0)?;
        }

        Ok(())
    }
}
