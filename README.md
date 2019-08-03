# moveslice

[![moveslice](https://img.shields.io/crates/v/moveslice.svg)](https://crates.io/crates/moveslice)
[![moveslice](https://docs.rs/moveslice/badge.svg)](https://docs.rs/crate/moveslice)


This crate contains functionality to move a slice within an array around.
It only uses safe functions, and acts efficiently by using the
[`split_at_mut`][split-at-mut] and
[`rotate_left`][rotate-left]/[`rotate_right`][rotate-right] functions.

This crate also has a focus on being `no_std`, to allow this functionality
in all cases where it is required.

The main feature this crate provides is implementing `moveslice` functions
for any and all slices/arrays. Note that this only works for slices - vectors
and other structures would need to be converted into a slice before using this function.

## Examples:

```rust
use moveslice::Moveslice;

let mut arr = [1,2,3,4,5,6,7,8,9];

// The following moves the slice 3..6 to index 1.
// In effect, it moves [4,5,6] over to where [2] is.
arr.moveslice(3..6, 1);
assert_eq!(arr, [1,4,5,6,2,3,7,8,9]);

// The following moves the slice 3..6 to index 6.
// In effect, it moves [6,2,3] over to where [7] is.
arr.moveslice(3..6, 6);
assert_eq!(arr, [1,4,5,7,8,9,6,2,3]);

// The following attempts to move the slice beyond boundaries.
// The index given is 7, which exists in the array, but the
// last element of the chunk will not fit (7 + 3 = 10 > 9).
// Therefore, the following should fail.
arr.moveslice(3..6, 7); // will panic

// Panicking on failure however can prove to be not ideal.
// If instead of panicking, you prefer a `Result`, use
// `try_moveslice`.
let res = arr.try_moveslice(3..6, 7);
assert!(res.is_err());

// Moveslice also comes with its own `Error` enum, to offer
// better debugging. Right now, there's only one error case.

// You could pass the destination as the same value as chunk.0.
// However this would mean nothing is moved.
// This doesn't panic, but it's a no-op.
arr.moveslice(0..3, 0);
```

[split-at-mut]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
[rotate-left]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left
[rotate-right]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right

License: MPL-2.0
