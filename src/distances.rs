use num_primitive::PrimitiveFloat;
use std::iter;

use crate::Point;

// Regular squared euclidean distance
pub fn squared_euclidean<T: PrimitiveFloat, const D: usize>(
    point1: &Point<T, D>,
    point2: &Point<T, D>,
) -> T {
    iter::zip(point1.p, point2.p)
        .map(|(a, b)| *a - b)
        .map(|x| x * x)
        .sum::<T>()
}

// Squared euclidean distance inside a periodic box
pub fn squared_euclidean_periodic<T: PrimitiveFloat, const D: usize>(
    point1: &Point<T, D>,
    point2: &Point<T, D>,
    boxsize: T,
) -> T {
    assert!(boxsize > T::MIN_POSITIVE);

    boxsize
}
