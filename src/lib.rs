use num_primitive::PrimitiveFloat;

pub use distances::*;
pub use point::{Point, points_from_slice};

pub mod distances;
pub mod point;
pub mod tree;

pub fn bounding_box<T: PrimitiveFloat, const D: usize>(
    points: &[Point<T, D>],
) -> Option<[[T; D]; 2]> {
    if points.is_empty() {
        return None;
    }

    let mut min = [T::MAX; D];
    let mut max = [T::MIN; D];

    // TODO: rewrite using iterators
    for point in points {
        for i in 0..D {
            min[i] = min[i].min(point.p[i]);
            max[i] = max[i].max(point.p[i]);
        }
    }

    Some([min, max])
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

    #[test]
    fn test_bounding_box() {
        let points = points_from_slice(&[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]);
        let expected = Some([[0.0, 0.0], [2.0, 2.0]]);
        assert_eq!(bounding_box(&points), expected);
    }
}
