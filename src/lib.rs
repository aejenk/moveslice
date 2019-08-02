#![no_std]
//! This crate contains a single function `moveslice`. Its purpose 
//! is to move a chunk within a slice around. It only uses safe functions,
//! and acts efficiently by using the 
//! [`split_at_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut)
//! and 
//! [`rotate_left`](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left)/
//! [`rotate_right`](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right)
//! functions.
//! 
//! # Examples:
//! 
//! ```
//! use moveslice::moveslice;
//! 
//! let mut arr = [1,2,3,4,5,6,7,8,9];
//! 
//! // The following moves the slice 3..6 to index 1.
//! // In effect, it moves [4,5,6] over to where [2] is.
//! moveslice(&mut arr, (3,6), 1);
//! assert_eq!(arr, [1,4,5,6,2,3,7,8,9]);
//! 
//! // The following moves the slice 3..6 to index 6.
//! // In effect, it moves [6,2,3] over to where [7] is.
//! moveslice(&mut arr, (3,6), 6);
//! assert_eq!(arr, [1,4,5,7,8,9,6,2,3]);
//! 
//! // The following attempts to move the slice beyond boundaries.
//! // The index given is 7, which exists in the array, but the 
//! // last element of the chunk will not fit (7 + 3 = 10 > 9).
//! // Therefore, the following should fail.
//! # #[should_panic]
//! # fn main() {
//! # let mut arr = [1,2,3,4,5,6,7,8,9];
//! let result = moveslice(&mut arr, (3,6), 7);
//! # }
//! 
//! // You could pass the destination as the same value as chunk.0.
//! // However this would mean nothing is moved.
//! // This doesn't panic, but it's a no-op.
//! moveslice(&mut arr, (0,3), 0);
//! ```

/// Moves a slice around in an array.
/// Works by splitting and rotating.
/// 
/// There are three parameters:
/// 
/// - `slice` : The slice to modify.
/// - `chunk` : A tuple with the boundaries of the chunk you want to move.
/// - `destination` : Where you want to move the chunk.
/// 
/// Note that the destination specifies where the *first* element of the chunk 
/// will be. As a result, its maximum value is not the length of the slice.
/// 
/// For example, if you have a slice with size 10, and you're moving a chunk of 
/// size 3 around, the maximum value for the destination is *10-3=* ***7***.
/// 
/// # Panics
/// Panics when the destination leads the chunk out of bounds.
/// 
/// In the example above, if I specify a destination of 8, the function will panic,
/// showing what would be the placement of the chunk, and the length of the slice.
/// 
/// ```should_panic
/// # use moveslice::moveslice;
/// # fn main() {
/// let mut arr = [1,2,3,4,5,6,7,8,9];
/// let result = moveslice(&mut arr, (3,6), 7); // will panic
/// # }
/// ```
pub fn moveslice<T>(slice: &mut [T], chunk: (usize, usize), destination: usize) {
    if destination > chunk.0 {
        let len = { slice.len() };
        let index1 = chunk.0;
        let index2 = destination;
        let chunksize = chunk.1 - chunk.0;

        let (_, mid) = slice.split_at_mut(index1);

        let mid = if destination + chunksize <= len {
            mid.split_at_mut(index2).0
        } else {
            panic!("Direction goes beyond slice [len = {}, destination = {}..{}]. ",
                    len, destination, destination + chunksize);
        };

        mid.rotate_left(chunk.1-chunk.0);
    } else if destination < chunk.0 {
        let index1 = destination;
        let index2 = chunk.1 - destination;

        let (_, mid) = slice.split_at_mut(index1);

        let mid = mid.split_at_mut(index2).0;

        mid.rotate_right(chunk.1-chunk.0);
    }
}