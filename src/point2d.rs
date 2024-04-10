use omega8::{Point8, D8_BASIS, D8_BASIS_DIVISOR};
use omega12::{Point12, D12_BASIS, D12_BASIS_DIVISOR};

pub struct Point2d(pub f64, pub f64);
pub type Tile2d = Vec<Point2d>; // use when outputting to 2d coords

pub fn get_point2d_list_from8(point8_list: &Vec<Point8>) -> Vec<Point2d> {
  return point8_list.iter().map(get_point2d_from8).collect();
}

pub fn get_point2d_from8(point8: &Point8) -> Point2d {
  return Point2d(
    (point8.x[0] as f64 + point8.x[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64),
    (point8.y[0] as f64 + point8.y[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64)
  );
}

pub fn get_point2d_list_from12(point12_list: &Vec<Point12>) -> Vec<Point2d> {
  return point12_list.iter().map(get_point2d_from12).collect();
}

pub fn get_point2d_from12(point12: &Point12) -> Point2d {
  return Point2d(
    (point12.x[0] as f64 + point12.x[1] as f64 * D12_BASIS[1])/(D12_BASIS_DIVISOR as f64),
    (point12.y[0] as f64 + point12.y[1] as f64 * D12_BASIS[1])/(D12_BASIS_DIVISOR as f64)
  );
}
