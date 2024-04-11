use omega8::Point8;
use omega12::Point12;

pub const SQUARE8: [usize; 4] = [
  2, 2, 2, 2
];

pub const OCTAGON8: [usize; 8] = [
  1, 1, 1, 1, 1, 1, 1, 1
];

pub const TRIANGLE12: [usize; 3] = [
  4, 4, 4
];

pub const HEXAGON12: [usize; 6] = [
  2, 2, 2, 2, 2, 2
];

pub fn get_square(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  return get_shape8(start_point, start_direction, &SQUARE8);
}

pub fn get_octagon(start_point: Point8, start_direction: usize) -> Vec<Point8> {
  return get_shape8(start_point, start_direction, &OCTAGON8);
}

pub fn get_triangle(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape12(start_point, start_direction, &TRIANGLE12);
}

pub fn get_hexagon(start_point: Point12, start_direction: usize) -> Vec<Point12> {
  return get_shape12(start_point, start_direction, &HEXAGON12);
}

pub fn get_shape8(start_point: Point8, start_direction: usize, shape: &[usize]) -> Vec<Point8> {
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

pub fn get_shape12(start_point: Point12, start_direction: usize, shape: &[usize]) -> Vec<Point12> {
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
