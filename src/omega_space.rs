use omega_coords::{OmegaSubCoordBasis, OmegaVector, OmegaSpace};

pub const SQRT2: f64 = std::f64::consts::SQRT_2;
pub const SQRT3: f64 = 1.7320508075688772935274463;

#[allow(dead_code)]
pub const OMEGA8_SUB_COORD_BASIS: OmegaSubCoordBasis<2> = OmegaSubCoordBasis{
  coefficients: [1.0, SQRT2],
  divisor: 2
};
#[allow(dead_code)]
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

#[allow(dead_code)]
pub const OMEGA12_SUB_COORD_BASIS: OmegaSubCoordBasis<2> = OmegaSubCoordBasis{
  coefficients: [1.0, SQRT3],
  divisor: 2
};
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_omega8_space() -> OmegaSpace<8, 2> {
  return OmegaSpace {
    unit_vectors: OMEGA8_UNIT_VECTORS,
    basis: OMEGA8_SUB_COORD_BASIS,
  };
}

#[allow(dead_code)]
pub fn get_omega12_space() -> OmegaSpace<12, 2> {
  return OmegaSpace {
    unit_vectors: OMEGA12_UNIT_VECTORS,
    basis: OMEGA12_SUB_COORD_BASIS,
  };
}

#[cfg(test)]
mod tests {
  use omega_coords::OmegaPoint;
  use super::*;

  #[test]
  fn test_omega8() {
    let space = get_omega8_space();
    let origin: OmegaPoint<2> = OmegaPoint::origin();
    let target = origin.plus(&space.unit_vectors[2]);
    let target_xy = target.to_xy_point(&space.basis);
    assert_eq!(target_xy.x, 0.0);
    assert_eq!(target_xy.y, 1.0);
  }

  #[test]
  fn test_omega12() {
    let space = get_omega12_space();
    let origin: OmegaPoint<2> = OmegaPoint::origin();
    let target = origin.plus(&space.unit_vectors[9]);
    let target_xy = target.to_xy_point(&space.basis);
    assert_eq!(target_xy.x, 0.0);
    assert_eq!(target_xy.y, -1.0);
  }
}
