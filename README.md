# moveslice
[![moveslice](https://img.shields.io/crates/v/moveslice.svg)](https://crates.io/crates/moveslice)
[![moveslice](https://docs.rs/moveslice/badge.svg)](https://docs.rs/crate/moveslice)

This crate contains a single function `moveslice`. Its purpose
is to move a chunk within a slice around. It only uses safe functions,
and acts efficiently by using the
[`split_at_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut)
and
[`rotate_left`](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left)/
[`rotate_right`](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right)
functions.

## Examples:

```rust
use moveslice::moveslice;

let mut arr = [1,2,3,4,5,6,7,8,9];

// The following moves the slice 3..6 to index 1.
// In effect, it moves [4,5,6] over to where [2] is.
moveslice(&mut arr, (3,6), 1);
assert_eq!(arr, [1,4,5,6,2,3,7,8,9]);

// The following moves the slice 3..6 to index 6.
// In effect, it moves [6,2,3] over to where [7] is.
moveslice(&mut arr, (3,6), 6);
assert_eq!(arr, [1,4,5,7,8,9,6,2,3]);

// The following attempts to move the slice beyond boundaries.
// The index given is 7, which exists in the array, but the
// last element of the chunk will not fit (7 + 3 = 10 > 9).
// Therefore, the following should fail.
let result = moveslice(&mut arr, (3,6), 7);

// You could pass the destination as the same value as chunk.0.
// However this would mean nothing is moved.
// Since it's not technically an error however, only a warning is logged.
moveslice(&mut arr, (0,3), 0);
```
