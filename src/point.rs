use num_primitive::PrimitiveFloat;

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

pub fn points_from_slice<'a, T: PrimitiveFloat, const D: usize>(
    slice: &'a [[T; D]],
) -> Vec<Point<'a, T, D>> {
    slice.iter().map(|p| Point::new(p)).collect()
}
