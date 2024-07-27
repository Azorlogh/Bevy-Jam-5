use std::mem::MaybeUninit;

use super::Ring;

pub struct IntoIter<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    i: usize,
    origin: usize,
}

impl<T, const N: usize> IntoIterator for Ring<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: Same code is in IntoIterator for [T; N]
        let data: [MaybeUninit<T>; N] = self.buf.map(|elem: T| MaybeUninit::new(elem));
        IntoIter {
            data,
            origin: self.head,
            i: 0,
        }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.i < N).then_some({
            let data = unsafe {
                self.data
                    .get_unchecked((self.origin + self.i) % N)
                    .assume_init_read()
            };
            self.i += 1;
            data
        })
    }
}
