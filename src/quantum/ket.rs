use crate::complex::Complex;
use alloc::format;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Ket {
    pub(crate) size: u32,
    pub(crate) vec: Vec<Complex>,
}

impl Ket {
    pub fn new(size: u32) -> Ket {
        let mut k = Ket::new_zero(size);
        #[cfg(not(test))]
        {
            *unsafe { k.vec.get_unchecked_mut(0) } = Complex::new(1.0, 0.0);
        }
        #[cfg(test)]
        {
            *{ k.vec.get_mut(0).unwrap() } = Complex::new(1.0, 0.0);
        }

        k
    }

    pub(crate) fn new_zero(size: u32) -> Ket {
        assert!(size > 0);
        let vec = Vec::from_iter((0..0x1 << size).map(|_| Complex::new(0.0, 0.0)));

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
