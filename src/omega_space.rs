use omega_coords::{OmegaSubCoordBasis, OmegaVector, OmegaSpace};

pub const SQRT2: f64 = std::f64::consts::SQRT_2;
pub const SQRT3: f64 = 1.7320508075688772935274463;

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
pub fn get_omega8() -> OmegaSpace<8, 2> {
  return OmegaSpace {
    unit_vectors: OMEGA8_UNIT_VECTORS,
  };
}

#[allow(dead_code)]
pub fn get_omega12() -> OmegaSpace<12, 2> {
  return OmegaSpace {
    unit_vectors: OMEGA12_UNIT_VECTORS,
  };
}
