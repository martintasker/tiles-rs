use omega_coords::{SQRT2, SQRT3, OmegaSubCoordBasis, OmegaVector};

#[allow(dead_code)]
pub const OMEGA8_SUB_COORD_BASIS: OmegaSubCoordBasis<2> = [
  0.0,
  SQRT2,
];
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
pub const OMEGA12_SUB_COORD_BASIS: OmegaSubCoordBasis<2> = [
  0.0,
  SQRT3,
];
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
pub struct Omega<const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  pub unit_vectors: [OmegaVector<BASIS_SIZE>; N_DIRECTIONS],
}

#[allow(dead_code)]
pub fn get_omega8() -> Omega<8, 2> {
  return Omega {
    unit_vectors: OMEGA8_UNIT_VECTORS,
  };
}

#[allow(dead_code)]
pub fn get_omega12() -> Omega<12, 2> {
  return Omega {
    unit_vectors: OMEGA12_UNIT_VECTORS,
  };
}
