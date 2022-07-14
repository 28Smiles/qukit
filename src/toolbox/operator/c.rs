use core::marker::PhantomData;
use core::ops::Mul;
use crate::complex::Complex;
use crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator;
use crate::toolbox::operator::controlled::Controlled;
use crate::toolbox::parameterized::Parameterized;
use crate::util::one::One;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate)struct C<
    const SIZE: usize,
    T: Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, V>> + Sized + Copy, V: Copy + Sized + Default + One + 'static
>(usize, T, PhantomData<V>)
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,;

impl<const SIZE: usize, T: Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, V>> + Sized + Copy, V: Copy + Sized + Default + One> C<SIZE, T, V>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    pub(crate) const fn new<I: ~const Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, IV>> + Sized + Copy, IV: Copy + Sized + One + Default>(wire: usize, inner: I) -> C<SIZE, I, IV> {
        C(wire, inner, PhantomData::default())
    }

    pub(crate)fn operator(self) -> ConstSizedUnitaryOperator<SIZE, V> {
        Controlled::<SIZE, V>::new(self.0, self.1).into()
    }
}

impl<const SIZE: usize, T: ~const Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, V>> + Sized + Copy, V: Copy + Sized + ~const Default + ~const One> const Into<ConstSizedUnitaryOperator<SIZE, V>> for C<SIZE, T, V>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    fn into(self) -> ConstSizedUnitaryOperator<SIZE, V> {
        Controlled::<SIZE, V>::new(self.0, self.1).into()
    }
}

impl<const SIZE: usize, T: ~const Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, V>> + Sized + Copy, V: Copy + Sized + ~const Default + ~const One> const Into<Controlled<SIZE, V>> for C<SIZE, T, V>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
{
    fn into(self) -> Controlled<SIZE, V> {
        Controlled::<SIZE, V>::new(self.0, self.1)
    }
}

impl<
    const SIZE: usize,
    O: ~const Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, OVT>>
        + ~const Parameterized<ConstSizedUnitaryOperator<{ SIZE - 1 }, PVT>, P>
        + Sized + Copy,
    P: ~const Into<ConstSizedUnitaryOperator<{ SIZE - 1 }, PVT>>
        + ~const Parameterized<ConstSizedUnitaryOperator<{ SIZE - 1 }, PVT>, P>
        + Sized + Copy,
    OVT: Copy + Sized + Default + One,
    PVT: Copy + Sized + Mul<Complex, Output = Complex> + Sync + Send + Default + One,
> const Parameterized<ConstSizedUnitaryOperator<SIZE, PVT>, C<SIZE, P, PVT>> for C<SIZE, O, OVT>
    where
        [(); 0x1 << SIZE]:,
        [(); 0x1 << { SIZE - 1 }]:,
        C<SIZE, P, PVT>: Into<ConstSizedUnitaryOperator<SIZE, PVT>>,
{
    fn parameterized(&self, theta: f64) -> C<SIZE, P, PVT> {
        C::<SIZE, P, PVT>::new(self.0, self.1.parameterized(theta))
    }
}
