use std::ops::RangeInclusive;

use crate::cubit::Cubit;
struct Cube<const N: usize> {
    // FIX: When you can start evaluating generics, change this Vec to use an array
    cubits: Vec<Cubit>,
}

impl<const N: usize> Cube<N> {
    fn new() -> Self {
        if N < 2 {
            panic!("Size of cube cannot be less than 2")
        }
        let size = N.pow(3) - (N - 2).pow(3);
        let v = Vec::with_capacity(size);
        // TODO: Use single for loop and modulo to figure out x, y, and z
        // TODO: Need to clone normalized orientation vectors for each cubit
        Cube { cubits: v }
    }
}
