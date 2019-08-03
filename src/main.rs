extern crate moveslice;
use moveslice::Moveslice;

fn main() {
    let mut arr = [1,2,3,4,5,6,7,8,9];
    let x = arr.try_moveslice(2..5, 4);

    println!("{:?}", arr);
}

// [1,2,3,4,5,6,7,8,9]
// Wanna shift [2,3,4] to [4].
// Split 1 : [1] - [2,3,4,5,6,7,8,9]
// Split 2 : [1] - [2,3,4,5,6,7] - [8,9]
//  INDEX 1 : 1
//  INDEX 2 : kinda 7, but actually 6.