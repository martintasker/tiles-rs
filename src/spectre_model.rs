use tesselation::TesselationBuilder;
use omega12::Point12;
use shapes::{Tile12, get_spectre};

pub fn get() -> Vec<Tile12> {
  let mut builder = TesselationBuilder::new();

  let s0 = get_spectre(Point12::origin(), 2);
  builder.add_tile(&s0);
  let s1 = get_spectre(s0[4], 3);
  builder.add_tile(&s1);
  let s2 = get_spectre(s1[4], 4);
  builder.add_tile(&s2);
  let s3 = get_spectre(s2[6], 4);
  builder.add_tile(&s3);
  let s4 = get_spectre(s2[8], 6);
  builder.add_tile(&s4);

  builder.build().tiles
}
