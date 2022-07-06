pub(crate) struct ConstIter(pub(crate) usize, pub(crate) usize);
impl const Iterator for ConstIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1 {
            None
        } else {
            let r = Some(self.0);
            self.0 += 1;
            r
        }
    }
}
