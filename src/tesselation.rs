pub struct Point2d(pub f64, pub f64);

pub fn get_square() -> Vec<Point2d> {
  let square = vec![
    Point2d(0.5, 0.5),
    Point2d(-0.5, 0.5),
    Point2d(-0.5, -0.5),
    Point2d(0.5, -0.5)
  ];
  return square;
}