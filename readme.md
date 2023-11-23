# Pinned Queue

Queue-like data structure whose elements can never move.

Inspired by [pinned_vec](https://github.com/wishawa/decurse/blob/main/pinned_vec).

### Example Usage
```rs
use pinned_queue::PinnedQueue;

let mut queue = PinnedQueue::new();
queue.push_back(1);
queue.push_back(2);
queue.pop_front();
queue.pop_front();
```
