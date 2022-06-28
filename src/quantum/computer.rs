use alloc::vec::Vec;
use alloc::vec;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use crate::complex::Complex;
use crate::quantum::ket::Ket;

pub struct QuantumComputer {
    state: Ket,
    pub(crate) seed: SmallRng,
}

impl QuantumComputer {
    pub fn new(q_bits: usize, seed: Option<u64>) -> QuantumComputer {
        QuantumComputer {
            state: Ket::new(q_bits),
            seed: SmallRng::seed_from_u64(seed.unwrap_or(42)),
        }
    }

    pub fn state(&self) -> &Vec<Complex> {
        &self.state.vec
    }

    pub fn amplitudes(&self) -> Vec<f64> {
        self.state.vec.iter().map(|c| c.amplitude()).collect::<Vec<f64>>()
    }

    pub fn probabilities(&self) -> Vec<f64> {
        let mut probalillites = vec![0.0; self.state.size as usize];
        for state_id in 0..self.state.vec.len() {
            for bit_id in 0..self.state.size {
                if state_id & (0x1 << bit_id) > 0 {
                    #[cfg(all(not(test), not(feature = "safe")))]
                    {
                        *unsafe { probalillites.get_unchecked_mut(bit_id as usize) } += unsafe { self.state.vec.get_unchecked(state_id) }.amplitude();
                    }
                    #[cfg(any(test, feature = "safe"))]
                    {
                        *{ probalillites.get_mut(bit_id as usize).unwrap() } += self.state.vec.get(state_id).unwrap().amplitude();
                    }
                }
            }
        }

        return probalillites
    }

    pub fn probability(&self, bit: usize) -> f64 {
        assert!(self.state.size > bit);
        let mut proballily = 0.0;
        let bit_m = 0x1 << bit;
        for state_id in 0..self.state.vec.len() {
            if state_id & bit_m > 0 {
                #[cfg(all(not(test), not(feature = "safe")))]
                {
                    proballily += unsafe { self.state.vec.get_unchecked(state_id) }.amplitude();
                }
                #[cfg(any(test, feature = "safe"))]
                {
                    proballily += self.state.vec.get(state_id).unwrap().amplitude();
                }
            }
        }

        return proballily
    }

    pub(crate) fn set_state(&mut self, ket: Ket) {
        self.state = ket;
    }

    pub(crate) fn get_state(&self) -> &Ket {
        &self.state
    }
}
