#![no_std]
//! This crate contains functionality to move a slice within an array around.
//! It only uses safe functions, and acts efficiently by using the
//! [`split_at_mut`][split-at-mut] and
//! [`rotate_left`][rotate-left]/[`rotate_right`][rotate-right] functions.
//! 
//! This crate also has a focus on being `no_std`, to allow this functionality
//! in all cases where it is required.
//! 
//! The main feature this crate provides is implementing `moveslice` functions
//! for any and all slices/arrays. In effect, it implements it on any type that 
//! also implements the AsMut<[T]> trait. This includes slices and vectors.
//! 
//! # Examples:
//! 
//! ```
//! use moveslice::{Moveslice, Error};
//! 
//! let mut arr = [1,2,3,4,5,6,7,8,9];
//! 
//! // The following moves the slice 3..6 to index 1.
//! // In effect, it moves [4,5,6] over to where [2] is.
//! arr.moveslice(3..6, 1);
//! assert_eq!(arr, [1,4,5,6,2,3,7,8,9]);
//! 
//! // The following moves the slice 3..6 to index 6.
//! // In effect, it moves [6,2,3] over to where [7] is.
//! arr.moveslice(3..6, 6);
//! assert_eq!(arr, [1,4,5,7,8,9,6,2,3]);
//! 
//! // The following attempts to move the slice beyond boundaries.
//! // The index given is 7, which exists in the array, but the 
//! // last element of the chunk will not fit (7 + 3 = 10 > 9).
//! // Therefore, the following should fail.
//! # #[should_panic]
//! # fn main() {
//! # let mut arr = [1,2,3,4,5,6,7,8,9];
//! arr.moveslice(3..6, 7); // will panic
//! # }
//! 
//! // Panicking on failure however can prove to be not ideal.
//! // If instead of panicking, you prefer a `Result`, use 
//! // `try_moveslice`.
//! let res = arr.try_moveslice(3..6, 7);
//! assert!(res.is_err());
//! 
//! // Moveslice also comes with its own `Error` enum, with diagnostic
//! // information to help debugging. The line before would have triggered
//! // an OutOfBoundsMove error. The following line would trigger the 
//! // InvalidBounds error.
//! let res = arr.try_moveslice(9..10, 7);
//! assert!(if let Err(Error::InvalidBounds{..}) = res {true} else {false});
//! 
//! // You could pass the destination as the same value as chunk.0.
//! // However this would mean nothing is moved.
//! // This doesn't panic, but it's a no-op.
//! arr.moveslice(0..3, 0);
//! ```
//! 
//! [split-at-mut]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
//! [rotate-left]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left
//! [rotate-right]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right

use core::ops::Bound::*;
use core::ops::RangeBounds;

/// This Error enum has a single variant, which is used to return additional information about
/// the out of bounds error, to help diagnostics.
/// 
/// Is used/returned by `try_moveslice`.
#[derive(Debug)]
pub enum Error {
    /// This error signifies an out of bounds error.
    /// It also contains the length of the slice, and 
    /// the supposed location of where the chunk would have been.
    /// 
    /// For example:
    /// `OutOfBoundsMove {len: 10, dest: (8,11)}`
    OutOfBoundsMove {
        /// The length of the array/slice being modified.
        len: usize,
        /// The location of where the chunk would have ended up.
        dest: (usize, usize)
    },

    /// This error signifies an invalid bounds error.
    /// If the bounds passed are already out of bounds, this 
    /// error is returned instead. This is to differentiate
    /// between the two out-of-bounds cases.
    InvalidBounds {
        // The length of the array/slice being modified.
        len: usize,
        // The effective bounds passed in.
        bounds: (usize, usize)
    }
}

/// A trait declaring the `moveslice` and `try_moveslice` functions.
/// Used to implement the functions on all slices. 
pub trait Moveslice<T, R> {
    /// Specifies the type of the destination index.
    type Target; 

    /// Specifies the errors being returned.
    type Err;

    /// Moves a slice within an array/slice around.
    /// 
    /// - `bounds` - specifies the range of where the subslice is. Examples: 3..5, 5..=8
    /// - `destination` - specifies where the subslice should be moved to.
    fn moveslice(&mut self, bounds: R, destination: Self::Target)
        where R: RangeBounds<usize>;

    /// Similar to `moveslice`, except it does not panic, returning a `Result` instead.
    fn try_moveslice(&mut self, bounds: R, destination: Self::Target) -> Result<(), Self::Err>
        where R: RangeBounds<usize>;
}

/// Implements the moveslice functions on all slices.
impl<T: 'static,R,A> Moveslice<T,R> for A where A: AsMut<[T]> {
    type Target = usize;
    type Err = Error;

    fn moveslice(&mut self, bounds: R, destination: Self::Target)
    where R: RangeBounds<usize> 
    {
        let res = self.try_moveslice(bounds, destination);
        if let Err(Error::OutOfBoundsMove{len, dest: (x,y)}) = res {
            panic!("Movement goes beyond bounds. [len = {}, destination = {}..{}]", len, x, y);
        }
        else if let Err(Error::InvalidBounds{len, bounds: (x,y)}) = res {
            panic!("Bounds passed go beyond slice length. [len = {}, bounds = {}..{}]", len, x, y);
        }
    }

    fn try_moveslice(&mut self, bounds: R, destination: Self::Target) -> Result<(), Self::Err>
    where R: RangeBounds<usize> 
    {
        let slice = self.as_mut();
        let startbound = bounds.start_bound();
        let endbound = bounds.end_bound();
        let x = if let Included(x) = startbound {*x} else {0};
        let y = if let Excluded(x) = endbound {*x}
                else if let Included(x) = endbound {x+1} 
                else {slice.len()};
        let chunk = (x,y);

        if chunk.0 > slice.len() || chunk.1 > slice.len() {
            return Err(Error::InvalidBounds {
                len: slice.len(),
                bounds: chunk
            });
        }

        if destination > chunk.0 {
            let chunksize = chunk.1 - chunk.0;
            let index1 = chunk.0;
            let index2 = destination + chunksize - index1;

            let (_, mid) = slice.split_at_mut(index1);

            let mid = if index2 <= mid.len() {
                mid.split_at_mut(index2).0
            } else {
                return Err(Error::OutOfBoundsMove {
                    len: slice.len(),
                    dest: (destination, destination + chunksize),
                });
            };

            mid.rotate_left(chunk.1-chunk.0);
        } else if destination < chunk.0 {
            let index1 = destination;
            let index2 = chunk.1 - destination;

            let (_, mid) = slice.split_at_mut(index1);

            let mid = mid.split_at_mut(index2).0;

            mid.rotate_right(chunk.1-chunk.0);
        }

        Ok(())
    }
}
