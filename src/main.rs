use num_primitive::PrimitiveFloat;

use fof::{Point, squared_distance};

fn main() {
    // everything should work when declaring as Vec<[f32; 3]> or Vec<[f64; 3]>
    let points = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 2.0],
        [-1.0, 3.0, 0.5],
        [2.5, -4.0, 0.0],
    ];

    println!("{:?}", points);

    let point1 = Point { p: &points[0] };
    let point2 = Point { p: &points[2] };
    println!("point1: {:?}", point1);
    println!("point2: {:?}", point2);
    println!("point1 == point2: {}", point1 == point2);
    println!("point1 == point1: {}", point1 == point1);
    println!("distance: {}", squared_distance(&point1, &point2));

    println!("{:?}", point1);

    let mut sq_dists = vec![0.0; (points.len() * points.len() - points.len()) / 2];
    let mut ind: usize = 0;
    for i in 0..points.len() {
        let point1 = Point { p: &points[i] };
        for j in i + 1..points.len() {
            let point2 = Point { p: &points[j] };
            sq_dists[ind] = squared_distance(&point1, &point2);
            ind += 1;
        }
    }
    dbg!(&sq_dists);
    println!(
        "min distance: {}, max distance: {}",
        sq_dists
            .iter()
            .copied()
            .reduce(PrimitiveFloat::min)
            .unwrap()
            .sqrt(),
        sq_dists
            .iter()
            .copied()
            .reduce(PrimitiveFloat::max)
            .unwrap()
            .sqrt()
    );
    dbg!(&sq_dists);
}
