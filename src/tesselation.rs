use omega8::Point8;
use omega8::get_octagon;
use omega8::get_square;

pub type Tile = Vec<Point8>; // use in the model

struct Tesselation {
  tiles: Vec<Tile>,
}

struct TesselationBuilder(Tesselation);

impl TesselationBuilder {
  fn new() -> Self {
    Self(Tesselation{tiles: Vec::new()})
  }

  fn build(self) -> Tesselation {
    self.0
  }

  fn add_tile(&mut self, tile: &Tile) -> &mut Self {
    self.0.tiles.push(tile.clone());
    self
  }
}

pub fn get_model() -> Vec<Tile> {
  let mut builder = TesselationBuilder::new();

  // construct an initial central octagon
  let centre_octagon = get_octagon(Point8{x: [-1, 0], y: [-1, -1]}, 0);
  builder.add_tile(&centre_octagon);

  // construct a set of squares surrounding the central octagon, on diagonals
  for [point_index, initial_direction] in [[1, 7], [3, 1], [5, 3], [7, 5]] {
    let anchor_point: Point8 = centre_octagon[point_index];
    let adjacent_square = get_square(anchor_point, initial_direction);
    builder.add_tile(&adjacent_square);
  }

  // construct a set of octagons surrounding the central octagon
  for [point_index, initial_direction] in [[2, 7], [4, 1], [6, 3], [0, 5]] {
    let anchor_point: Point8 = centre_octagon[point_index];
    let adjacent_octagon = get_octagon(anchor_point, initial_direction);
    builder.add_tile(&adjacent_octagon);
  }

  builder.build().tiles
}
