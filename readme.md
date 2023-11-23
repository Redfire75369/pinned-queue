# Pinned Queue

Queue-like data structure whose elements can never move.

Inspired by [pinned_vec](https://github.com/wishawa/decurse/blob/main/pinned_vec).

### Example Usage
```rs
use pinned_queue::PinnedQueue;
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
```
