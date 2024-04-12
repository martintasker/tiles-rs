pub type OmegaSubCoords<const N: usize> = [i32; N];
pub type OmegaSubCoordBasis<const N: usize> = [f64; N];

pub fn sub_coord_sum<const N: usize>(a: &OmegaSubCoords<N>, b: &OmegaSubCoords<N>) -> OmegaSubCoords<N> {
  let mut res = a.clone();
  for i in 0..N {
    res[i] += b[i];
  }
  res
}

pub fn coord_projected<const N: usize>(coords: &OmegaSubCoords<N>, basis: &OmegaSubCoordBasis<N>) -> f64 {
  let mut res = 0.0;
  for i in 0..N {
    res += coords[i] as f64 * basis[i];
  }
  res
}

pub struct OmegaVector<const N: usize> {
  pub dx: OmegaSubCoords<N>,
  pub dy: OmegaSubCoords<N>,
}

pub struct OmegaPoint<const N: usize> {
  pub x: OmegaSubCoords<N>,
  pub y: OmegaSubCoords<N>,
}

pub struct XYPoint {
  pub x: f64,
  pub y: f64,
}

#[allow(dead_code)]
impl<const N: usize> OmegaPoint<N> {
  pub fn plus(&self, v: &OmegaVector<N>) -> Self {
    OmegaPoint {
      x: sub_coord_sum(&self.x, &v.dx),
      y: sub_coord_sum(&self.y, &v.dy),
    }
  }

  pub fn to_xy_point(&self, basis: &OmegaSubCoordBasis<N>) -> XYPoint {
    return XYPoint {
      x: coord_projected(&self.x, basis),
      y: coord_projected(&self.y, basis)
    }
  }
}

pub struct OmegaSpace<const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  pub unit_vectors: [OmegaVector<BASIS_SIZE>; N_DIRECTIONS],
}

impl<const N_DIRECTIONS: usize, const BASIS_SIZE: usize> OmegaSpace<N_DIRECTIONS, BASIS_SIZE> {
  #[allow(dead_code)]
  pub fn n_directions(self) -> usize {
    return N_DIRECTIONS;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sub_coord_sum() {
    assert_eq!(sub_coord_sum(&[1, 2], &[3, 4]), [4, 6]);
  }

  #[test]
  fn test_coord_projected() {
    assert_eq!(coord_projected(&[1, 2], &[1.0, 1.25]), 3.5);
  }

  #[test]
  fn test_vec_plus_point() {
    let p0 = OmegaPoint{x: [1, 2], y: [2, 1]};
    let v = OmegaVector{dx: [3, 4], dy: [4, 3]};
    let p1 = p0.plus(&v);
    assert_eq!(p1.x, [4, 6]);
    assert_eq!(p1.y, [6, 4]);
  }
}
