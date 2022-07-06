use core::ops::Deref;

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum SCow<T: Sized + Copy + 'static> {
    Owned(T),
    Borrowed(&'static T),
}

impl<T: Sized + Copy + 'static> const Deref for SCow<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            SCow::Owned(t) => t,
            SCow::Borrowed(t) => *t,
        }
    }
}

impl<T: Sized + Copy + 'static> const From<T> for SCow<T> {
    fn from(t: T) -> Self {
        SCow::Owned(t)
    }
}

impl<T: Sized + Copy + 'static> const From<&'static T> for SCow<T> {
    fn from(t: &'static T) -> Self {
        SCow::Borrowed(t)
    }
}
