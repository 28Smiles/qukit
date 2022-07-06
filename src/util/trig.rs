use const_trig::ToRadians;

#[inline(always)]
pub(crate) const fn const_sin(x: f64) -> f64 {
    const_trig::sin(x.radians(), None)
}

#[inline(always)]
pub(crate) const fn const_cos(x: f64) -> f64 {
    const_trig::cos(x.radians(), None)
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;
    use crate::util::trig::{const_cos, const_sin};

    #[test]
    fn test_sin() {
        for i in -512..512 {
            let r = const_sin((i as f64) / 128.0 * core::f64::consts::PI);
            let v0 = libm::sin((i as f64) / 128.0 * core::f64::consts::PI);
            approx_eq!(f64, v0, r, epsilon = 0.00000003);
        }
    }

    #[test]
    fn test_cos() {
        for i in -512..512 {
            let r = const_cos((i as f64) / 128.0 * core::f64::consts::PI);
            let v0 = libm::cos((i as f64) / 128.0 * core::f64::consts::PI);
            approx_eq!(f64, v0, r, epsilon = 0.00000003);
        }
    }
}
