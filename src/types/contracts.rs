pub trait Assign<T> {
    fn assign_to(self, dest: &mut T);
}

impl<T> Assign<T> for T {
    #[inline(always)]
    fn assign_to(self, dest: &mut T) {
        *dest = self;
    }
}
