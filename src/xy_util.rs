use xy_point::XYPoint;
use omega8::{Point8, D8_BASIS, D8_BASIS_DIVISOR};
use omega12::{Point12, D12_BASIS, D12_BASIS_DIVISOR};

pub type Tile2d = Vec<XYPoint>; // use when outputting to 2d coords

pub fn get_xy_point_list_from8(point8_list: &Vec<Point8>) -> Vec<XYPoint> {
  return point8_list.iter().map(get_xy_point_from8).collect();
}

pub fn get_xy_point_from8(point8: &Point8) -> XYPoint {
  return XYPoint{
    x: (point8.x[0] as f64 + point8.x[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64),
    y: (point8.y[0] as f64 + point8.y[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64)
  };
}

pub fn get_xy_point_list_from12(point12_list: &Vec<Point12>) -> Vec<XYPoint> {
  return point12_list.iter().map(get_xy_point_from12).collect();
}

pub fn get_xy_point_from12(point12: &Point12) -> XYPoint {
  return XYPoint{
    x: (point12.x[0] as f64 + point12.x[1] as f64 * D12_BASIS[1])/(D12_BASIS_DIVISOR as f64),
    y: (point12.y[0] as f64 + point12.y[1] as f64 * D12_BASIS[1])/(D12_BASIS_DIVISOR as f64)
  };
}

pub fn is_all_inside(points: &Vec<XYPoint>) -> bool {
  points.iter().all(is_inside)
}

const HALF_SIZE: f64 = 7.9;

pub fn is_inside(point: &XYPoint) -> bool {
  f64::abs(point.x) < HALF_SIZE && f64::abs(point.y) < HALF_SIZE
}
