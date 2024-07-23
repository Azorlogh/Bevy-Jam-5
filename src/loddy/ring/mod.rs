mod into_iter;
mod iter;
mod iter_mut;

use std::{
	mem::{ManuallyDrop, MaybeUninit},
	ops::Range,
	ptr,
};

#[derive(Clone, Copy, Debug)]
pub struct Ring<T, const N: usize> {
	buf: [T; N],
	head: usize,
}

impl<T: Default, const N: usize> Default for Ring<T, N> {
	fn default() -> Self {
		let arr = unsafe {
			let mut arr: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
			for e in &mut arr {
				*e = MaybeUninit::new(T::default());
			}
			std::mem::transmute_copy(&arr)
		};
		Self { buf: arr, head: 0 }
	}
}

impl<T: Default, const N: usize> From<[T; N]> for Ring<T, N> {
	fn from(buf: [T; N]) -> Self {
		Self { buf, head: 0 }
	}
}

impl<T: Default, const N: usize> Ring<T, N> {
	pub fn shift_right(&mut self) -> T {
		self.head = (self.head + N - 1) % N;
		std::mem::replace(&mut self.buf[self.head], T::default())
	}

	pub fn shift_left(&mut self) -> T {
		let r = std::mem::replace(&mut self.buf[self.head], T::default());
		self.head = (self.head + 1) % N;
		r
	}
}

impl<T, const N: usize> Ring<T, N> {
	pub fn rotate_right(&mut self) {
		self.head = (self.head + N - 1) % N;
	}

	pub fn rotate_left(&mut self) {
		self.head = (self.head + 1) % N;
	}

	#[inline]
	unsafe fn buffer_range(&mut self, range: Range<usize>) -> *mut [T] {
		unsafe {
			ptr::slice_from_raw_parts_mut(
				self.buf.as_mut_ptr().add(range.start),
				range.end - range.start,
			)
		}
	}

	pub fn to_array(self) -> [T; N] {
		// SAFETY: ManuallyDrop<T> has the same layout as T
		let mut buf: [ManuallyDrop<T>; N] = unsafe { ptr::read(std::mem::transmute(&self.buf)) };
		// SAFETY: MaybeUninit<T> has the same layout as T
		let mut arr: [MaybeUninit<T>; N] =
			unsafe { ptr::read(std::mem::transmute(&mut MaybeUninit::<[T; N]>::uninit())) };
		for i in 0..N {
			// SAFETY: Math
			let r = unsafe { ManuallyDrop::take(&mut buf[(self.head + i) % N]) };
			arr[i] = MaybeUninit::new(r);
		}
		// SAFETY: Everything is now initialized
		unsafe { ptr::read(std::mem::transmute(&mut arr)) }
	}

	pub fn as_slices(&self) -> (&[T], &[T]) {
		(&self.buf[self.head..N], &self.buf[0..self.head])
	}

	pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
		unsafe {
			(
				&mut *self.buffer_range(self.head..N),
				&mut *self.buffer_range(0..self.head),
			)
		}
	}
}

impl<T, const N: usize> std::ops::Index<usize> for Ring<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.buf[(self.head + index) % N]
	}
}

impl<T, const N: usize> std::ops::IndexMut<usize> for Ring<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.buf[(self.head + index) % N]
	}
}

impl<T: Default + PartialEq, const N: usize> PartialEq<[T]> for Ring<T, N> {
	fn eq(&self, other: &[T]) -> bool {
		(0..N).all(|i| self.buf[(self.head + i) % N] == other[i])
	}
}

#[cfg(test)]
mod tests {
	use super::Ring;

	#[test]
	fn test_shift() {
		let mut ring: Ring<u32, 5> = [0, 1, 2, 3, 4].into();
		assert_eq!(ring, *[0, 1, 2, 3, 4].as_slice());

		assert_eq!(ring.shift_right(), 4);
		assert_eq!(ring, *[0, 0, 1, 2, 3].as_slice());

		assert_eq!(ring.shift_left(), 0);
		assert_eq!(ring.shift_left(), 0);
		assert_eq!(ring.shift_left(), 1);
		assert_eq!(ring, *[2, 3, 0, 0, 0].as_slice());

		ring.rotate_left();
		assert_eq!(ring, *[3, 0, 0, 0, 2].as_slice());

		ring.shift_right();
		assert_eq!(ring, *[0, 3, 0, 0, 0].as_slice());
	}

	#[test]
	fn test_rotate() {
		let mut ring: Ring<u32, 5> = [0, 1, 2, 3, 4].into();
		assert_eq!(ring, *[0, 1, 2, 3, 4].as_slice());

		ring.rotate_right();
		assert_eq!(ring, *[4, 0, 1, 2, 3].as_slice());

		ring.rotate_left();
		assert_eq!(ring, *[0, 1, 2, 3, 4].as_slice());
	}

	#[test]
	fn test_to_array() {
		let mut ring: Ring<u32, 5> = [0, 1, 2, 3, 4].into();
		ring.rotate_right();
		ring.rotate_right();
		ring.rotate_right();
		assert_eq!(ring.clone().to_array(), [2, 3, 4, 0, 1]);

		ring.rotate_left();
		assert_eq!(ring.to_array(), [3, 4, 0, 1, 2]);
	}

	#[test]
	fn iteration() {
		let mut ring: Ring<u32, 5> = [0, 1, 2, 3, 4].into();
		assert!(Iterator::eq(ring.iter(), [0, 1, 2, 3, 4].iter()));

		assert!(Iterator::eq((&ring).into_iter(), [0, 1, 2, 3, 4].iter()));

		assert!(Iterator::eq(
			(&mut ring).into_iter(),
			[0, 1, 2, 3, 4].iter()
		));

		assert!(Iterator::eq(ring.into_iter(), [0, 1, 2, 3, 4].into_iter()));
	}
}
