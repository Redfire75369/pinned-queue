use std::collections::VecDeque;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr;

pub struct Block<T>(VecDeque<T>);

impl<T> Block<T> {
	fn new(capacity: usize) -> Block<T> {
		Block(VecDeque::with_capacity(capacity))
	}

	fn is_empty(&self) -> bool {
		self.0.len() == 0
	}

	fn get(&self, index: usize) -> Option<Pin<&T>> {
		self.0.get(index).map(|p| unsafe { Pin::new_unchecked(p) })
	}

	fn get_mut(&mut self, index: usize) -> Option<Pin<&mut T>> {
		self.0.get_mut(index).map(|p| unsafe { Pin::new_unchecked(p) })
	}

	fn push_back(&mut self, item: T) {
		assert!(self.0.len() < self.0.capacity());
		self.0.push_back(item);
	}

	fn pop_front(&mut self) {
		self.0.front_mut().map(|p| unsafe {
			ptr::drop_in_place(p);
		});
		let _ = ManuallyDrop::new(self.0.pop_front());
	}

	fn replace(&mut self, index: usize, item: T) {
		*self.0.get_mut(index).unwrap() = item;
	}
}

#[derive(Default)]
pub struct PinnedQueue<T> {
	blocks: VecDeque<Block<T>>,
	head: usize,
	len: usize,
}

impl<T> PinnedQueue<T> {
	pub const fn new() -> PinnedQueue<T> {
		PinnedQueue { blocks: VecDeque::new(), head: 0, len: 0 }
	}

	/// Returns the number of elements in the queue.
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// Provides a pinned reference to the element at the given index.
	pub fn get(&self, index: usize) -> Option<Pin<&T>> {
		if index > self.len {
			None
		} else {
			let (outer, inner) = split_index(self.head, index);
			self.blocks[outer].get(inner)
		}
	}

	/// Provides a pinned mutable reference to the element at the given index.
	pub fn get_mut(&mut self, index: usize) -> Option<Pin<&mut T>> {
		if index > self.len {
			None
		} else {
			let (outer, inner) = split_index(self.head, index);
			self.blocks[outer].get_mut(inner)
		}
	}

	/// Provides a pinned mutable reference to the last element.
	pub fn last_mut(&mut self) -> Option<Pin<&mut T>> {
		self.get_mut(self.len)
	}

	/// Appends an element to the end of the queue.
	pub fn push_back(&mut self, item: T) {
		let head_outer = outer_index(self.head);
		let outer = outer_index(self.head + self.len);

		if outer - head_outer >= self.blocks.len() {
			self.blocks.push_back(Block::new(1 << outer));
		}

		self.blocks[outer - head_outer].push_back(item);
		self.len += 1;
	}

	/// Removes the first element and returns `true`, or `false` if the queue is empty.
	pub fn pop_front(&mut self) -> bool {
		if self.is_empty() {
			return false;
		}

		self.len -= 1;
		self.head += 1;
		self.blocks[0].pop_front();

		if self.blocks[0].is_empty() {
			self.blocks.pop_front();
		}
		true
	}

	/// Replaces the element at the given index with another one.
	pub fn replace(&mut self, index: usize, item: T) {
		let (outer, inner) = split_index(self.head, index);
		self.blocks[outer].replace(inner, item);
	}
}

fn outer_index(index: usize) -> usize {
	(usize::BITS - (index + 1).leading_zeros() - 1) as usize
}

fn split_index(head: usize, index: usize) -> (usize, usize) {
	let outer = outer_index(index + head);
	let inner = (head + index + 1) & (!(1 << outer));
	let head_outer = outer_index(head);
	let head_inner = (head + 1) & (!(1 << head_outer));
	if head_inner > inner {
		(outer - head_outer - 1, head_inner - inner)
	} else {
		(outer - head_outer, inner - head_inner)
	}
}

#[cfg(test)]
mod tests {
	use crate::{outer_index, split_index};

	#[test]
	fn outer() {
		assert_eq!(0, outer_index(0));
		assert_eq!(1, outer_index(1));
		assert_eq!(1, outer_index(2));
		assert_eq!(2, outer_index(3));
		assert_eq!(16, outer_index(65535));
	}

	#[test]
	fn split() {
		assert_eq!((0, 0), split_index(0, 0));
		assert_eq!((1, 0), split_index(0, 1));
		assert_eq!((1, 1), split_index(0, 2));

		assert_eq!((0, 0), split_index(1, 0));
		assert_eq!((0, 1), split_index(1, 1));
		assert_eq!((1, 0), split_index(1, 2));

		assert_eq!((3, 1), split_index(2, 15));
	}
}
