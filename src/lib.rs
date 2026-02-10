use num_primitive::PrimitiveFloat;
use std::iter;

// A n-dim Point
// generic over either f32 or f64 and number of dimensions
#[derive(Debug, PartialEq)]
pub struct Point<'a, T: PrimitiveFloat, const D: usize> {
    pub p: &'a [T; D],
}

impl<'a, T: PrimitiveFloat, const D: usize> Point<'a, T, D> {
    pub fn new(p: &'a [T; D]) -> Self {
        Point { p }
    }
}

// Regular squared euclidean distance
pub fn squared_distance<T: PrimitiveFloat, const D: usize>(
    point1: &Point<T, D>,
    point2: &Point<T, D>,
) -> T {
    iter::zip(point1.p, point2.p)
        .map(|(a, b)| *a - b)
        .map(|x| x * x)
        .sum::<T>()
}

// Squared euclidean distance inside a periodic box
pub fn squared_distance_periodic<T: PrimitiveFloat, const D: usize>(
    point1: &Point<T, D>,
    point2: &Point<T, D>,
    boxsize: T,
) -> T {
    assert!(boxsize > T::MIN_POSITIVE);

    boxsize
}

// Angular distance on a sphere
pub fn angular_distance<T: PrimitiveFloat>(point1: &Point<T, 2>, point2: &Point<T, 2>) -> T {
    point1.p[0]
}

// binary tree containing points
#[derive(Debug)]
pub struct TreeNode<'a, T: PrimitiveFloat, const D: usize> {
    pub point: Point<'a, T, D>,
    pub left: Option<&'a TreeNode<'a, T, D>>,
    pub right: Option<&'a TreeNode<'a, T, D>>,
}

pub fn make_tree<'a, T: PrimitiveFloat, const D: usize>(points: &Vec<[T; D]>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_2d() {
        let _point = Point::new(&[1.0, 2.0]);
    }

    #[test]
    fn point_3d() {
        let _point = Point::new(&[1.0, 2.0, 3.0]);
    }

    #[test]
    fn point_f32() {
        let _point = Point::new(&[1.0f32, 2.0f32, 3.0f32]);
    }

    #[test]
    fn point_f64() {
        let _point = Point::new(&[1.0f64, 2.0f64, 3.0f64]);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Point::new(&[0.0, 1.0, 2.0]), Point::new(&[0.0, 1.0, 2.0]));
        assert_ne!(Point::new(&[0.0, 1.0, 2.0]), Point::new(&[0.0, 0.5, 1.0]));
    }
}
