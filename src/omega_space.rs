use omega_coords::{OmegaSubCoordBasis, OmegaVector, OmegaPoint};

use crate::omega_coords::XYPoint;

pub const SQRT2: f64 = std::f64::consts::SQRT_2;
pub const SQRT3: f64 = 1.7320508075688772935274463;

pub const OMEGA8_BASIS: OmegaSubCoordBasis<2> = OmegaSubCoordBasis{
  coefficients: [1.0, SQRT2],
  divisor: 2
};
pub const OMEGA8_UNIT_VECTORS: [OmegaVector<2>; 8] = [
  OmegaVector {dx: [2, 0], dy: [0, 0]},
  OmegaVector {dx: [0, 1], dy: [0, 1]},
  OmegaVector {dx: [0, 0], dy: [2, 0]},
  OmegaVector {dx: [0, -1], dy: [0, 1]},
  OmegaVector {dx: [-2, 0], dy: [0, 0]},
  OmegaVector {dx: [0, -1], dy: [0, -1]},
  OmegaVector {dx: [0, 0], dy: [-2, 0]},
  OmegaVector {dx: [0, 1], dy: [0, -1]},
];
pub const OMEGA12_BASIS: OmegaSubCoordBasis<2> = OmegaSubCoordBasis{
  coefficients: [1.0, SQRT3],
  divisor: 2
};
pub const OMEGA12_UNIT_VECTORS: [OmegaVector<2>; 12] = [
  OmegaVector {dx: [2, 0], dy: [0, 0]},
  OmegaVector {dx: [0, 1], dy: [1, 0]},
  OmegaVector {dx: [1, 0], dy: [0, 1]},
  OmegaVector {dx: [0, 0], dy: [2, 0]},
  OmegaVector {dx: [-1, 0], dy: [0, 1]},
  OmegaVector {dx: [0, -1], dy: [1, 0]},
  OmegaVector {dx: [-2, 0], dy: [0, 0]},
  OmegaVector {dx: [0, -1], dy: [-1, 0]},
  OmegaVector {dx: [-1, 0], dy: [0, -1]},
  OmegaVector {dx: [0, 0], dy: [-2, 0]},
  OmegaVector {dx: [1, 0], dy: [0, -1]},
  OmegaVector {dx: [0, 1], dy: [-1, 0]},
];

pub struct OmegaSpace<const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  pub unit_vectors: [OmegaVector<BASIS_SIZE>; N_DIRECTIONS],
  pub basis: OmegaSubCoordBasis<BASIS_SIZE>,
}
#[allow(dead_code)]
pub const OMEGA8_SPACE: OmegaSpace<8, 2> = OmegaSpace {
  unit_vectors: OMEGA8_UNIT_VECTORS,
  basis: OMEGA8_BASIS,
};
#[allow(dead_code)]
pub const OMEGA12_SPACE: OmegaSpace<12, 2> = OmegaSpace {
  unit_vectors: OMEGA12_UNIT_VECTORS,
  basis: OMEGA12_BASIS,
};

impl<const N_DIRECTIONS: usize, const BASIS_SIZE: usize> OmegaSpace<N_DIRECTIONS, BASIS_SIZE> {
  #[allow(dead_code)]
  pub fn n_directions(self) -> usize {
    return N_DIRECTIONS;
  }

  #[allow(dead_code)]
  pub fn direction_plus(&self, direction: usize, delta: i32) -> usize {
    (((direction + N_DIRECTIONS) as i32 + delta) as usize) % N_DIRECTIONS
  }
}

#[allow(dead_code)]
pub struct OmegaSpaceVector<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  space: &'a OmegaSpace<N_DIRECTIONS, BASIS_SIZE>,
  vector: OmegaVector<BASIS_SIZE>,
}

impl <'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> OmegaSpaceVector<'a, N_DIRECTIONS, BASIS_SIZE> {
  #[allow(dead_code)]
  pub fn new(space: &'a OmegaSpace<N_DIRECTIONS, BASIS_SIZE>) -> Self {
    OmegaSpaceVector{
      space: &space,
      vector: OmegaVector::zero(),
    }
  }
}

#[allow(dead_code)]
pub struct OmegaSpacePoint<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  space: &'a OmegaSpace<N_DIRECTIONS, BASIS_SIZE>,
  point: OmegaPoint<BASIS_SIZE>,
}

impl <'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> OmegaSpacePoint<'a, N_DIRECTIONS, BASIS_SIZE> {
  #[allow(dead_code)]
  pub fn new(space: &'a OmegaSpace<N_DIRECTIONS, BASIS_SIZE>) -> Self {
    OmegaSpacePoint{
      space: &space,
      point: OmegaPoint::origin(),
    }
  }

  #[allow(dead_code)]
  pub fn plus(&self, vector: &OmegaSpaceVector<'a, N_DIRECTIONS, BASIS_SIZE>) -> Self {
    OmegaSpacePoint {
      space: &self.space,
      point: self.point.plus(&vector.vector)
    }
  }

  #[allow(dead_code)]
  pub fn plus_unit_in_direction(&self, direction: usize) -> Self {
    OmegaSpacePoint {
      space: &self.space,
      point: self.point.plus(&self.space.unit_vectors[direction]),
    }
  }

  #[allow(dead_code)]
  pub fn to_xy_point(&self) -> XYPoint {
    self.point.to_xy_point(&self.space.basis)
  }
}

#[cfg(test)]
mod tests {
  use omega_coords::OmegaPoint;
  use super::*;

  #[test]
  fn test_direction_plus() {
    let space: OmegaSpace<12, 0> = OmegaSpace{
      unit_vectors: core::array::from_fn(|_| OmegaVector{dx: [], dy: []}),
      basis: OmegaSubCoordBasis{ coefficients: [], divisor: 2},
    };
    assert_eq!(space.direction_plus(0, 1), 1);
    assert_eq!(space.direction_plus(0, -1), 11);
    assert_eq!(space.direction_plus(8, 5), 1);
    assert_eq!(space.direction_plus(3, -5), 10);
  }

  #[test]
  fn test_omega8() {
    let space = OMEGA8_SPACE;
    let origin: OmegaPoint<2> = OmegaPoint::origin();
    let target = origin.plus(&space.unit_vectors[2]);
    let target_xy = target.to_xy_point(&space.basis);
    assert_eq!(target_xy.x, 0.0);
    assert_eq!(target_xy.y, 1.0);
  }

  #[test]
  fn test_omega12() {
    let space = OMEGA12_SPACE;
    let origin: OmegaPoint<2> = OmegaPoint::origin();
    let target = origin.plus(&space.unit_vectors[9]);
    let target_xy = target.to_xy_point(&space.basis);
    assert_eq!(target_xy.x, 0.0);
    assert_eq!(target_xy.y, -1.0);
  }

  #[test]
  fn test_space_point() {
    let space = OMEGA12_SPACE;
    let point12 = OmegaSpacePoint::new(&space);
    let xy_point = point12.to_xy_point();
    assert_eq!(xy_point.x, 0.0);
    assert_eq!(xy_point.y, 0.0);

    let xy_point_11 =
      OmegaSpacePoint::new(&space)
        .plus_unit_in_direction(0)
        .plus_unit_in_direction(3)
        .to_xy_point();
    assert_eq!(xy_point_11.x, 1.0);
    assert_eq!(xy_point_11.y, 1.0);
  }
}
