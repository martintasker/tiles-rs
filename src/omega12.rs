// 8 directions
pub const N_DIRECTIONS: usize = 12;

// each coordinate is an integer multiple of 1 plus integer multiple of sqrt 3, all divided by 2
const SQRT3: f64 = 1.7320508075688772935274463;
pub type D12Basis = [i32; 2];
pub const D12_BASIS: [f64; 2] = [
  1.0,
  SQRT3,
];
pub const D12_BASIS_DIVISOR: i32 = 2;

// so here is our direction struct, along with its 12 unit instances
pub struct Dir12 {
  dx: D12Basis,
  dy: D12Basis,
}
pub const DIR12_UNIT_VECTORS: [Dir12; N_DIRECTIONS] = [
  Dir12 {dx: [2, 0], dy: [0, 0]},
  Dir12 {dx: [0, 1], dy: [1, 0]},
  Dir12 {dx: [1, 0], dy: [0, 1]},
  Dir12 {dx: [0, 0], dy: [2, 0]},
  Dir12 {dx: [-1, 0], dy: [0, 1]},
  Dir12 {dx: [0, -1], dy: [1, 0]},
  Dir12 {dx: [-2, 0], dy: [0, 0]},
  Dir12 {dx: [0, -1], dy: [-1, 0]},
  Dir12 {dx: [-1, 0], dy: [0, -1]},
  Dir12 {dx: [0, 0], dy: [-2, 0]},
  Dir12 {dx: [1, 0], dy: [0, -1]},
  Dir12 {dx: [0, 1], dy: [-1, 0]},
];

// points are on the same omega12 basis, obtained by adding vectors to starting-points
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point12 {
  pub x: D12Basis,
  pub y: D12Basis,
}

impl Point12 {
  pub fn add_unit_in_direction(&self, d: usize) -> Self {
    let dir = &DIR12_UNIT_VECTORS[d % N_DIRECTIONS];
    return Point12 {
      x: [self.x[0] + dir.dx[0], self.x[1] + dir.dx[1]],
      y: [self.y[0] + dir.dy[0], self.y[1] + dir.dy[1]]
    };
  }
}

// triangle: turn left 120 degrees at each node
pub const TRIANGLE12: [usize; 3] = [
  4, 4, 4
];

// hexagon: turn left 60 degrees at each node
pub const HEXAGON12: [usize; 6] = [
  2, 2, 2, 2, 2, 2
];

pub fn get_triangle(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape(start_point, start_direction, &TRIANGLE12);
}

pub fn get_hexagon(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape(start_point, start_direction, &HEXAGON12);
}

pub fn get_shape(start_point: Point12, start_direction: usize, shape: &[usize]) -> Vec<Point12> {
  let mut res: Vec<Point12> = Vec::new();
  let mut p = start_point.clone();
  let mut d = start_direction;
  for dir12 in shape {
    res.push(p.clone());
    p = p.add_unit_in_direction(d);
    d += dir12;
  }
  return res;
}

#[cfg(test)]
mod tests {
  use super::*;

  impl Point12 {
    pub fn origin() -> Self {
      Point12 {
        x: [0, 0],
        y: [0, 0],
      }
    }
  }

  #[test]
  fn test_unit_0() {
      assert_eq!(Point12::origin().add_unit_in_direction(0), Point12{x: [2, 0], y: [0, 0]});
  }
}