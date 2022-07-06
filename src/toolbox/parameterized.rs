use crate::runtime::unitary::UnitaryOperator;

pub(crate)trait Parameterized<V: UnitaryOperator, T: Into<V> + Parameterized<V, T>> {
    fn parameterized(&self, theta: f64) -> T;
}
