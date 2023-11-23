use std::pin::Pin;
use std::ptr;

use pinned_queue::PinnedQueue;

#[test]
fn test() {
	let mut queue = PinnedQueue::new();
	queue.push_back(1);
	let p = queue.get(0).unwrap();
	assert_eq!(1, *p);
	let p1 = Pin::into_inner(p) as *const _;

	queue.push_back(2);
	let p = queue.get(0).unwrap();
	assert_eq!(1, *p);
	assert!(ptr::eq(p1, Pin::into_inner(p) as *const _));

	queue.push_back(3);

	let p1 = Pin::into_inner(queue.get_mut(2).unwrap()) as *mut _;
	queue.push_back(4);
	queue.push_back(5);

	queue.pop_front();
	queue.pop_front();

	let p = queue.get_mut(0).unwrap();
	assert_eq!(3, *p);
	assert!(ptr::eq(p1, Pin::into_inner(p) as *const _));
}
