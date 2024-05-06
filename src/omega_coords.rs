use xy_point::XYPoint;

// a subcoordinate is the x or y coordinate for a point
// the basis is
// * a list of real numbers, which we dot-multiply by the subcoord integers
// * an integer divisor, which divides the whole result

/// real subcoord basis with integer fraction
pub struct OmegaSubCoordBasis<const N: usize> {
  pub coefficients: [f64; N],
  pub divisor: i32,
}

/// integer subcoord instance
pub type OmegaSubCoords<const N: usize> = [i32; N];

/// project an omega subcoord into an x or y subcoord
pub fn coord_projected<const N: usize>(coords: &OmegaSubCoords<N>, basis: &OmegaSubCoordBasis<N>) -> f64 {
  let mut res = 0.0;
  for i in 0..N {
    res += coords[i] as f64 * basis.coefficients[i];
  }
  res / (basis.divisor as f64)
}

/// add a single subcoord pair
pub fn add_sub_coords<const N: usize>(a: &OmegaSubCoords<N>, b: &OmegaSubCoords<N>) -> OmegaSubCoords<N> {
  let mut res = a.clone();
  for i in 0..N {
    res[i] += b[i];
  }
  res
}

/// omega point is x, y subcoords
#[derive(Clone, PartialEq, Debug)]
pub struct OmegaPoint<const N: usize> {
  pub x: OmegaSubCoords<N>,
  pub y: OmegaSubCoords<N>,
}

/// omega vector is also in omega space, like point, but with dx, dy
pub struct OmegaVector<const N: usize> {
  pub dx: OmegaSubCoords<N>,
  pub dy: OmegaSubCoords<N>,
}

/// origin point
impl<const N: usize> OmegaPoint<N> {
  pub fn origin() -> Self {
    Self{x: [0; N], y: [0; N]}
  }
}

/// null vector
impl<const N: usize> OmegaVector<N> {
  pub fn zero() -> Self {
    Self{dx: [0; N], dy: [0; N]}
  }
}

// operations on points
impl<const N: usize> OmegaPoint<N> {
  /// add vector
  pub fn plus(&self, v: &OmegaVector<N>) -> Self {
    OmegaPoint {
      x: add_sub_coords(&self.x, &v.dx),
      y: add_sub_coords(&self.y, &v.dy),
    }
  }

  /// to XYPoint
  pub fn to_xy_point(&self, basis: &OmegaSubCoordBasis<N>) -> XYPoint {
    return XYPoint {
      x: coord_projected(&self.x, basis),
      y: coord_projected(&self.y, basis)
    }
  }
}

// operations on vectors
impl<const N: usize> OmegaVector<N> {
  /// add vector
  pub fn plus(&self, v: &OmegaVector<N>) -> Self {
    OmegaVector {
      dx: add_sub_coords(&self.dx, &v.dx),
      dy: add_sub_coords(&self.dy, &v.dy),
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

  #[test]
  fn test_vec_plus_vec() {
    let v0 = OmegaVector{dx: [1, 2], dy: [2, 1]};
    let dv = OmegaVector{dx: [3, 4], dy: [4, 3]};
    let v1 = v0.plus(&dv);
    assert_eq!(v1.dx, [4, 6]);
    assert_eq!(v1.dy, [6, 4]);
  }
}
