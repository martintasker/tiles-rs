use tesselation::TesselationBuilder;
use omega12::{Point12, get_hexagon};

pub type Tile12 = Vec<Point12>;

pub fn get() -> Vec<Tile12> {
  let mut builder = TesselationBuilder::new();

  // construct an initial central hexagon
  let centre_hexagon = get_hexagon(Point12{x: [-1, 0], y: [0, -1]}, 0);
  builder.add_tile(&centre_hexagon);

  builder.build().tiles
}
