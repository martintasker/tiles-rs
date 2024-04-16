pub struct OmegaSubCoordBasis<const N: usize> {
  pub coefficients: [f64; N],
  pub divisor: i32,
}

pub type OmegaSubCoords<const N: usize> = [i32; N];

pub fn coord_projected<const N: usize>(coords: &OmegaSubCoords<N>, basis: &OmegaSubCoordBasis<N>) -> f64 {
  let mut res = 0.0;
  for i in 0..N {
    res += coords[i] as f64 * basis.coefficients[i];
  }
  res / (basis.divisor as f64)
}

pub fn add_sub_coords<const N: usize>(a: &OmegaSubCoords<N>, b: &OmegaSubCoords<N>) -> OmegaSubCoords<N> {
  let mut res = a.clone();
  for i in 0..N {
    res[i] += b[i];
  }
  res
}

pub struct OmegaPoint<const N: usize> {
  pub x: OmegaSubCoords<N>,
  pub y: OmegaSubCoords<N>,
}

impl<const N: usize> OmegaPoint<N> {
  pub fn origin() -> Self {
    Self{x: [0; N], y: [0; N]}
  }
}

pub struct XYPoint {
  pub x: f64,
  pub y: f64,
}

pub struct OmegaVector<const N: usize> {
  pub dx: OmegaSubCoords<N>,
  pub dy: OmegaSubCoords<N>,
}

impl<const N: usize> OmegaVector<N> {
  pub fn zero() -> Self {
    Self{dx: [0; N], dy: [0; N]}
  }
}

impl<const N: usize> OmegaPoint<N> {
  pub fn plus(&self, v: &OmegaVector<N>) -> Self {
    OmegaPoint {
      x: add_sub_coords(&self.x, &v.dx),
      y: add_sub_coords(&self.y, &v.dy),
    }
  }

  pub fn to_xy_point(&self, basis: &OmegaSubCoordBasis<N>) -> XYPoint {
    return XYPoint {
      x: coord_projected(&self.x, basis),
      y: coord_projected(&self.y, basis)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sub_coord_sum() {
    assert_eq!(add_sub_coords(&[1, 2], &[3, 4]), [4, 6]);
  }

  #[test]
  fn test_coord_projected() {
    assert_eq!(
      coord_projected(
        &[1, 2],
        &OmegaSubCoordBasis{coefficients: [1.0, 1.25], divisor: 1}
      ),
      3.5
    );
    assert_eq!(
      coord_projected(
        &[1, 2],
        &OmegaSubCoordBasis{coefficients: [1.0, 1.25], divisor: 2}
      ),
      1.75
    );
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
