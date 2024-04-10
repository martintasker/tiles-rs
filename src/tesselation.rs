pub struct Tesselation<TTile> {
  pub tiles: Vec<TTile>,
}

pub struct TesselationBuilder<TTile>(Tesselation<TTile>);

impl<TTile: Clone> TesselationBuilder<TTile> {
  pub fn new() -> Self {
    Self(Tesselation{tiles: Vec::new()})
  }

  pub fn build(self) -> Tesselation<TTile> {
    self.0
  }

  pub fn add_tile(&mut self, tile: &TTile) -> &mut Self {
    self.0.tiles.push(tile.clone());
    self
  }
}
