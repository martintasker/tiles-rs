use omega8::Point8;
use omega8::D8_BASIS;
use omega8::D8_BASIS_DIVISOR;

pub struct Point2d(pub f64, pub f64);
pub type Tile2d = Vec<Point2d>; // use when outputting to 2d coords

pub fn get_point2d_list(point8_list: &Vec<Point8>) -> Vec<Point2d> {
  return point8_list.iter().map(get_point2d).collect();
}

pub fn get_point2d(point8: &Point8) -> Point2d {
  return Point2d(
    (point8.x[0] as f64 + point8.x[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64),
    (point8.y[0] as f64 + point8.y[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64)
  );
}
