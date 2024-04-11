use omega8::Point8;
use omega12::Point12;

pub type Tile8 = Vec<Point8>;
pub type Tile12 = Vec<Point12>;

pub const SQUARE8: [isize; 4] = [
  2, 2, 2, 2
];

pub fn get_square(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  return get_shape8(start_point, start_direction, &SQUARE8);
}

pub const OCTAGON8: [isize; 8] = [
  1, 1, 1, 1, 1, 1, 1, 1
];

pub fn get_octagon(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  return get_shape8(start_point, start_direction, &OCTAGON8);
}

pub const TRIANGLE12: [isize; 3] = [
  4, 4, 4
];

pub fn get_triangle(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape12(start_point, start_direction, &TRIANGLE12);
}

pub const HEXAGON12: [isize; 6] = [
  2, 2, 2, 2, 2, 2
];

pub fn get_hexagon(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape12(start_point, start_direction, &HEXAGON12);
}

pub const SPECTRE12: [isize; 14] = [
  2, -3, 2, 3, 2, -3, 2, 0, 2, 3, -2, 3, -2, 3
];

pub fn get_spectre(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape12(start_point, start_direction, &SPECTRE12);
}

pub fn get_shape8(start_point: Point8, start_direction: usize, shape: &[isize]) -> Vec<Point8> {
  let mut res: Vec<Point8> = Vec::new();
  let mut p = start_point.clone();
  let mut d = start_direction;
  for dir8 in shape {
    res.push(p.clone());
    p = p.add_unit_in_direction(d);
    d = (d as isize + dir8 + 8) as usize % 8;
  }
  return res;
}

pub fn get_shape12(start_point: Point12, start_direction: usize, shape: &[isize]) -> Vec<Point12> {
  let mut res: Vec<Point12> = Vec::new();
  let mut p = start_point.clone();
  let mut d = start_direction;
  for dir12 in shape {
    res.push(p.clone());
    p = p.add_unit_in_direction(d);
    d = (d as isize + dir12 + 12) as usize % 12;
  }
  return res;
}
