// 8 directions
pub const N_DIRECTIONS: usize = 8;

// each coordinate is an integer multiple of 1 plus integer multiple of sqrt 2, all divided by 2
pub type D8Basis = [i32; 2];
pub const D8_BASIS: [f64; 2] = [
  1.0,
  std::f64::consts::SQRT_2
];
pub const D8_BASIS_DIVISOR: i32 = 2;

// so here is our direction struct, along with its 8 unit instances
pub struct Dir8 {
  dx: D8Basis,
  dy: D8Basis,
}
pub const DIR8_UNIT_VECTORS: [Dir8; N_DIRECTIONS] = [
  Dir8 {dx: [2, 0], dy: [0, 0]},
  Dir8 {dx: [0, 1], dy: [0, 1]},
  Dir8 {dx: [0, 0], dy: [2, 0]},
  Dir8 {dx: [0, -1], dy: [0, 1]},
  Dir8 {dx: [-2, 0], dy: [0, 0]},
  Dir8 {dx: [0, -1], dy: [0, -1]},
  Dir8 {dx: [0, 0], dy: [-2, 0]},
  Dir8 {dx: [0, 1], dy: [0, -1]},
];

// points are on the same omega8 basis, obtained by adding vectors to starting-points
#[derive(Clone, Copy)]
pub struct Point8 {
  pub x: D8Basis,
  pub y: D8Basis,
}

impl Point8 {
  pub fn add_unit_in_direction(&self, d: usize) -> Self {
    let dir = &DIR8_UNIT_VECTORS[d % N_DIRECTIONS];
    return Point8 {
      x: [self.x[0] + dir.dx[0], self.x[1] + dir.dx[1]],
      y: [self.y[0] + dir.dy[0], self.y[1] + dir.dy[1]]
    };
  }
}

// square: turn left 90 degrees at each node
pub const SQUARE8: [usize; 4] = [
  2, 2, 2, 2
];

// octagon: turn left 45 degrees at each node
pub const OCTAGON8: [usize; 8] = [
  1, 1, 1, 1, 1, 1, 1, 1
];

pub fn get_square(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  return get_shape(start_point, start_direction, &SQUARE8);
}

pub fn get_octagon(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  return get_shape(start_point, start_direction, &OCTAGON8);
}

pub fn get_shape(start_point: Point8, start_direction: usize, shape: &[usize]) -> Vec<Point8> {
  let mut res: Vec<Point8> = Vec::new();
  let mut p = start_point.clone();
  let mut d = start_direction;
  for dir8 in shape {
    res.push(p.clone());
    p = p.add_unit_in_direction(d);
    d += dir8;
  }
  return res;
}
