use omega_space::OmegaSpacePoint;

#[allow(dead_code)]
pub struct Tile<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  points: &'a Vec<OmegaSpacePoint<'a, N_DIRECTIONS, BASIS_SIZE>>,
}

impl<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> Tile<'a, N_DIRECTIONS, BASIS_SIZE> {
  #[allow(dead_code)]
  pub fn new(points: &'a Vec<OmegaSpacePoint<N_DIRECTIONS, BASIS_SIZE>>) -> Self {
    Tile { points }
  }

  #[allow(dead_code)]
  pub fn get_points(&self) -> &'a Vec<OmegaSpacePoint<N_DIRECTIONS, BASIS_SIZE>> {
    &self.points
  }
}

pub struct Tesselation<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> {
  tiles: Vec<Tile<'a, N_DIRECTIONS, BASIS_SIZE>>,
}

impl<'a, const N_DIRECTIONS: usize, const BASIS_SIZE: usize> Tesselation<'a, N_DIRECTIONS, BASIS_SIZE> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Tesselation { tiles: Vec::new() }
  }

  #[allow(dead_code)]
  pub fn get_tiles(&self) -> &'a Vec<Tile<N_DIRECTIONS, BASIS_SIZE>> {
    &self.tiles
  }

  #[allow(dead_code)]
  pub fn add(&mut self, tile: Tile<'a, N_DIRECTIONS, BASIS_SIZE>) -> &Self {
    self.tiles.push(tile);
    self
  }
}

#[cfg(test)]
mod tests {
  use omega_space::{OMEGA8_SPACE, OmegaSpacePoint};
  use omega_shapes::get_square;

  use super::*;

  #[test]
  fn test_get_square() {
    let square = get_square(OmegaSpacePoint::new(&OMEGA8_SPACE), 0);
    let tile = Tile::new(&square);
    assert_eq!(tile.get_points().len(), 4);
  }

  #[test]
  fn test_build_tesselation() {
    let square = get_square(OmegaSpacePoint::new(&OMEGA8_SPACE), 0);
    let tile0 = Tile::new(&square);
    let mut tesselation: Tesselation<8, 2> = Tesselation::new();
    assert_eq!(tesselation.get_tiles().len(), 0);
    tesselation.add(tile0);
    assert_eq!(tesselation.get_tiles().len(), 1);
  }
}
