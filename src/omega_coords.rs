pub const SQRT2: f64 = std::f64::consts::SQRT_2;
pub const SQRT3: f64 = 1.7320508075688772935274463;

pub type OmegaSubCoordBasis<const N: usize> = [f64; N];
pub type OmegaSubCoords<const N: usize> = [i32; N];

pub fn sub_coord_sum<const N: usize>(a: &OmegaSubCoords<N>, b: &OmegaSubCoords<N>) -> OmegaSubCoords<N> {
  let mut res = a.clone();
  for (i, vb) in b.iter().enumerate() {
    res[i] += vb;
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

#[allow(dead_code)]
impl<const N: usize> OmegaPoint<N> {
  pub fn plus(&self, v: &OmegaVector<N>) -> Self {
    OmegaPoint {
      x: sub_coord_sum(&self.x, &v.dx),
      y: sub_coord_sum(&self.y, &v.dy),
    }
  }
}
