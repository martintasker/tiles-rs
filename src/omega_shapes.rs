use omega_space::OmegaSpacePoint;

pub const SQUARE8: [i32; 4] = [
  2, 2, 2, 2
];
pub const OCTAGON8: [i32; 8] = [
  1, 1, 1, 1, 1, 1, 1, 1
];
pub const TRIANGLE12: [i32; 3] = [
  4, 4, 4
];
pub const HEXAGON12: [i32; 6] = [
  2, 2, 2, 2, 2, 2
];
pub const SQUARE12: [i32; 4] = [
  3, 3, 3, 3
];
pub const SPECTRE12: [i32; 14] = [
  2, -3, 2, 3, 2, -3, 2, 0, 2, 3, -2, 3, -2, 3
];

#[allow(dead_code)]
pub struct ShapeTemplate {
  deltas: Vec<i32>,
}

#[allow(dead_code)]
pub struct Tile<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  points: Vec<OmegaSpacePoint<'a, N_DIRECTIONS, BASIS_SIZE>>,
}

#[allow(dead_code)]
pub fn get_square<'a>(start_point: OmegaSpacePoint<'a, 8, 2>, start_direction: usize) -> Vec<OmegaSpacePoint<8, 2>> {
  return get_shape(start_point, start_direction, &SQUARE8);
}
#[allow(dead_code)]
pub fn get_octagon<'a>(start_point: OmegaSpacePoint<'a, 8, 2>, start_direction: usize) -> Vec<OmegaSpacePoint<8, 2>> {
  return get_shape(start_point, start_direction, &OCTAGON8);
}

#[allow(dead_code)]
pub fn get_triangle<'a>(start_point: OmegaSpacePoint<'a, 12, 2>, start_direction: usize) -> Vec<OmegaSpacePoint<12, 2>> {
  return get_shape(start_point, start_direction, &TRIANGLE12);
}
#[allow(dead_code)]
pub fn get_square12<'a>(start_point: OmegaSpacePoint<'a, 12, 2>, start_direction: usize) -> Vec<OmegaSpacePoint<12, 2>> {
  return get_shape(start_point, start_direction, &SQUARE12);
}
#[allow(dead_code)]
pub fn get_hexagon<'a>(start_point: OmegaSpacePoint<'a, 12, 2>, start_direction: usize) -> Vec<OmegaSpacePoint<12, 2>> {
  return get_shape(start_point, start_direction, &HEXAGON12);
}
#[allow(dead_code)]
pub fn get_spectre<'a>(start_point: OmegaSpacePoint<'a, 12, 2>, start_direction: usize) -> Vec<OmegaSpacePoint<12, 2>> {
  return get_shape(start_point, start_direction, &SPECTRE12);
}

pub fn get_shape<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize>(
  start_point: OmegaSpacePoint<'a, N_DIRECTIONS, BASIS_SIZE>,
  start_direction: usize,
  shape: &[i32]
) -> Vec<OmegaSpacePoint<'a, N_DIRECTIONS, BASIS_SIZE>> {
  let space = start_point.get_space();
  let mut res: Vec<OmegaSpacePoint<'a, N_DIRECTIONS, BASIS_SIZE>> = Vec::new();
  let mut p = start_point.clone();
  let mut direction = start_direction;
  for direction_change in shape {
    res.push(p.clone());
    p = p.plus_unit_in_direction(direction);
    direction = space.direction_plus(direction, *direction_change);
  }
  return res;
}
