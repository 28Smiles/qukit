use crate::complex::Complex;
use crate::error::{QuantumError, Result};
use alloc::format;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::ops::{Deref, DerefMut};
use alloc::vec;
use alloc::sync::Arc;
use bit_array::BitArray;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use spin::Mutex;

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
pub(crate)struct Ket {
    size: usize,
    pub(crate) vec: Vec<Complex>,
    pub(crate) seed: Arc<Mutex<SmallRng>>,
}

impl Ket {
    pub(crate) fn new(size: usize) -> Result<Ket> {
        #[cfg(feature = "wasm-bindgen")]
        {
            Ket::new_with_seed(size, Arc::new(Mutex::new(SmallRng::seed_from_u64(
                (js_sys::Math::random() * u64::MAX as f64) as u64
            ))))
        }
        #[cfg(all(feature = "std", not(feature = "wasm-bindgen")))]
        {
            Ket::new_with_seed(size, Arc::new(Mutex::new(
                SmallRng::from_rng(rand::prelude::thread_rng()).unwrap()
            )))
        }
        #[cfg(all(not(feature = "wasm-bindgen"), not(feature = "std")))]
        {
            Ket::new_with_seed(size, Arc::new(Mutex::new(SmallRng::seed_from_u64(42))))
        }
    }

    pub(crate) fn new_with_seed(size: usize, seed: Arc<Mutex<SmallRng>>) -> Result<Ket> {
        if size < 1 {
            return Err(QuantumError(format!("Size must be greater than 0, but was {}", size)))
        }
        let mut k = Ket {
            size,
            vec: vec![Complex::zero(); 0x1 << size],
            seed
        };
        *k.vec.get_mut(0).unwrap() = Complex::new(1.0, 0.0);

        Ok(k)
    }

    /// Create a new Ket with the same size and rng, but all zero.
    pub(crate) fn new_from(ket: &Ket) -> Ket {
        let vec = vec![Complex::zero(); ket.vec.len()];

        Ket { size: ket.size, vec, seed: ket.seed.clone() }
    }

    pub(crate) fn amplitudes(&self) -> Vec<f64> {
        return self
            .vec
            .iter()
            .map(|c| {
                let a = c.abs();
                a * a
            })
            .collect();
    }

    pub(crate) fn probabilities(&self) -> Vec<f64> {
        let mut probabilities = vec![0.0; self.size as usize];
        for state_id in 0..self.vec.len() {
            for bit_id in 0..self.size {
                if state_id & (0x1 << bit_id) > 0 {
                    *{ probabilities.get_mut(bit_id as usize).unwrap() } += self.vec.get(state_id).unwrap().amplitude();
                }
            }
        }

        return probabilities
    }

    pub(crate) fn probability(&self, bit: usize) -> f64 {
        assert!(self.size > bit);
        let mut probability = 0.0;
        let bit_m = 0x1 << bit;
        for state_id in 0..self.vec.len() {
            if state_id & bit_m > 0 {
                probability += self.vec.get(state_id).unwrap().amplitude();
            }
        }

        return probability
    }

    pub(crate) fn state(&self) -> &Vec<Complex> {
        &self.vec
    }

    #[inline(always)]
    pub(crate) fn size(&self) -> usize {
        self.size
    }
}

impl PartialEq for Ket {
    fn eq(&self, other: &Self) -> bool {
        self.vec.eq(&other.vec)
    }
}

impl Display for Ket {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for (i, v) in self.vec.iter().enumerate() {
            let s = format!("{:#0width$b}", i, width = self.size as usize + 2);
            let (_, s) = s.split_at(2);
            writeln!(f, "|{}> = {}", s, v)?;
        }

        Ok(())
    }
}
