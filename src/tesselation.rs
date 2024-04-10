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

  // add squares and octagons around the edge
  for [centre_octagon_point_index, initial_direction] in [[2, 7], [4, 1], [6, 3], [0, 5]] {
    let mut layer_anchor_point = centre_octagon[centre_octagon_point_index];

    // build layers in the given direction
    for layer in 1..=8 {
      // start with anchor octagon and square
      let layer_anchor_octagon = get_octagon(layer_anchor_point, initial_direction);
      builder.add_tile(&layer_anchor_octagon);
      let square_anchor_point: Point8 = layer_anchor_octagon[7];
      let layer_anchor_square = get_square(square_anchor_point, (initial_direction + 2) % 8);
      builder.add_tile(&layer_anchor_square);

      // then, for all layers except the first one, build the tail as needed
      let mut tail_anchor_point = layer_anchor_square[3];
      for _ in 2..=layer {
        let adjacent_octagon = get_octagon(tail_anchor_point, (initial_direction + 2) % 8);
        builder.add_tile(&adjacent_octagon);
        let square_anchor_point: Point8 = adjacent_octagon[5];
        let adjacent_square = get_square(square_anchor_point, (initial_direction + 2) % 8);
        builder.add_tile(&adjacent_square);
        tail_anchor_point = adjacent_square[3];
      }
      layer_anchor_point = layer_anchor_octagon[3];
    }
  }

  builder.build().tiles
}
