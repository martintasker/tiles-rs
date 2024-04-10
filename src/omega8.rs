/*
  Basis vector = [1, sqrt 2]
  Denominator = 2
  Lengths = [1, sqrt 2]
*/

const N_DIRECTIONS: usize = 8;
pub type D8Basis = [i32; 2];
pub const D8_BASIS: [f64; 2] = [1.0, std::f64::consts::SQRT_2];
pub const D8_BASIS_DIVISOR: i32 = 2;

pub struct Dir8 {
  dx: D8Basis,
  dy: D8Basis,
}

pub const DIR8_UNITS: [Dir8; N_DIRECTIONS] = [
  Dir8 {dx: [2, 0], dy: [0, 0]},
  Dir8 {dx: [0, 1], dy: [0, 1]},
  Dir8 {dx: [0, 0], dy: [2, 0]},
  Dir8 {dx: [0, -1], dy: [0, 1]},
  Dir8 {dx: [-2, 0], dy: [0, 0]},
  Dir8 {dx: [0, -1], dy: [0, -1]},
  Dir8 {dx: [0, 0], dy: [-2, 0]},
  Dir8 {dx: [0, 1], dy: [0, -1]},
];

pub const OCTAGON8: [usize; 8] = [
  1, 1, 1, 1, 1, 1, 1, 1
];

pub fn get_octagon(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  let mut res: Vec<Point8> = Vec::new();
  let mut p = start_point.clone();
  let mut d = start_direction;
  for dir8 in OCTAGON8 {
    res.push(p.clone());
    p = p.add_unit_in_direction(d);
    d += dir8;
  }
  return res;
}

#[derive(Clone)]
pub struct Point8 {
  pub x: D8Basis,
  pub y: D8Basis,
}

impl Point8 {
  pub fn add_unit_in_direction(&self, d: usize) -> Self {
    let dir = &DIR8_UNITS[d % N_DIRECTIONS];
    return Point8 {
      x: [self.x[0] + dir.dx[0], self.x[1] + dir.dx[1]],
      y: [self.y[0] + dir.dy[0], self.y[1] + dir.dy[1]]
    };
  }
}

pub struct Point2d(pub f64, pub f64);

pub fn get_point2d_list(point8_list: Vec<Point8>) -> Vec<Point2d> {
  return point8_list.iter().map(get_point2d).collect();
}

pub fn get_point2d(point8: &Point8) -> Point2d {
  return Point2d(
    (point8.x[0] as f64 + point8.x[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64),
    (point8.y[0] as f64 + point8.y[1] as f64 * D8_BASIS[1])/(D8_BASIS_DIVISOR as f64)
  );
}
