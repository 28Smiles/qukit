use core::fmt::{Display, Formatter};
use core::ops::{Add, Mul};
use libm::sqrt;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi, into_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize, serde::Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl const Add<Complex> for Complex {
    type Output = Complex;

    #[inline(always)]
    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re() + rhs.re(),
            im: self.im() + rhs.im(),
        }
    }
}

impl const Mul<Complex> for Complex {
    type Output = Complex;

    #[inline(always)]
    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re() * rhs.re() - self.im() * rhs.im(),
            im: self.re() * rhs.im() + self.im() * rhs.re(),
        }
    }
}

impl const Mul<Complex> for f64 {
    type Output = Complex;

    #[inline(always)]
    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::new(rhs.re() * self, rhs.im() * self)
    }
}

impl const Add<Complex> for f64 {
    type Output = Complex;

    #[inline(always)]
    fn add(self, rhs: Complex) -> Self::Output {
        Complex::new(rhs.re() + self, rhs.im())
    }
}

impl const Mul<f64> for Complex {
    type Output = Complex;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs.mul(self)
    }
}

impl const Add<f64> for Complex {
    type Output = Complex;

    #[inline(always)]
    fn add(self, rhs: f64) -> Self::Output {
        rhs.add(self)
    }
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Complex {
        Complex {
            re,
            im,
        }
    }

    pub const fn zero() -> Complex {
        Complex::new(0.0, 0.0)
    }

    #[inline(always)]
    pub const fn re(&self) -> f64 {
        self.re
    }

    #[inline(always)]
    pub const fn im(&self) -> f64 {
        self.im
    }

    #[inline(always)]
    pub fn abs(&self) -> f64 {
        sqrt(self.amplitude())
    }

    #[inline(always)]
    pub const fn amplitude(&self) -> f64 {
        self.re() * self.re() + self.im() * self.im()
    }

    #[inline(always)]
    pub const fn conjugate(&self) -> Complex {
        Complex::new(self.re(), -self.im())
    }

    #[inline(always)]
    pub const fn mul_e(&self, other: &Complex) -> Complex {
        Complex::new(self.re() * other.re(), self.im() * other.re())
    }

    #[inline(always)]
    pub const fn div_e(&self, other: &Complex) -> Complex {
        Complex::new(self.re() / other.re(), self.im() / other.re())
    }

    #[inline(always)]
    pub fn abs_e(&self) -> Complex {
        Complex::new(self.re().abs(), self.im().abs())
    }

    #[inline(always)]
    pub fn nan_to_zero(&self) -> Complex {
        Complex::new(if self.re() == f64::NAN { 0.0 } else { 0.0 }, if self.im() == f64::NAN { 0.0 } else { 0.0 })
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} + {}i", self.re(), self.im())
    }
}

impl const Default for Complex {
    fn default() -> Self {
        Complex::zero()
    }
}

#[cfg(test)]
mod test {
    use crate::complex::Complex;
    use float_cmp::{ApproxEq, F64Margin};

    impl ApproxEq for Complex {
        type Margin = F64Margin;
        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let margin = margin.into();
            self.re().approx_eq(other.re(), margin) && self.im().approx_eq(other.im(), margin)
        }
    }
}
