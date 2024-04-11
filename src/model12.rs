use tesselation::TesselationBuilder;
use omega12::Point12;
use shapes::{Tile12, get_hexagon, get_triangle};

pub fn get() -> Vec<Tile12> {
  let mut builder = TesselationBuilder::new();

  // construct an initial central hexagon
  let centre_hexagon = get_hexagon(Point12{x: [-1, 0], y: [0, -1]}, 0);
  builder.add_tile(&centre_hexagon);

  // add triangles and hexagons around the edge
  for [centre_hexagon_point_index, initial_direction] in [[2, 10], [3, 0], [4, 2], [5, 4], [0, 6], [1, 8]] {
    let mut layer_anchor_point = centre_hexagon[centre_hexagon_point_index];

    // build layers in the given direction
    for layer in 1..=9 {
      // start with anchor octagon and square
      let layer_anchor_hexagon = get_hexagon(layer_anchor_point, initial_direction);
      builder.add_tile(&layer_anchor_hexagon);
      let inner_triangle_anchor_point: Point12 = layer_anchor_hexagon[0];
      let inner_triangle = get_triangle(inner_triangle_anchor_point, (initial_direction + 4) % 12);
      builder.add_tile(&inner_triangle);
      let outer_triangle_anchor_point: Point12 = layer_anchor_hexagon[4];
      let outer_triangle = get_triangle(outer_triangle_anchor_point, (initial_direction + 6) % 12);
      builder.add_tile(&outer_triangle);

      // then, for all layers except the first one, build the tail as needed
      let mut tail_anchor_point = inner_triangle[2];
      for _ in 2..=layer {
        let adjacent_hexagon = get_hexagon(tail_anchor_point, (initial_direction + 2) % 12);
        builder.add_tile(&adjacent_hexagon);
        let inner_triangle_anchor_point: Point12 = adjacent_hexagon[5];
        let inner_triangle = get_triangle(inner_triangle_anchor_point, (initial_direction + 4) % 12);
        builder.add_tile(&inner_triangle);
        let outer_triangle_anchor_point: Point12 = adjacent_hexagon[3];
        let outer_triangle = get_triangle(outer_triangle_anchor_point, (initial_direction + 6) % 12);
        builder.add_tile(&outer_triangle);
        tail_anchor_point = inner_triangle[2];
      }

      layer_anchor_point = layer_anchor_hexagon[3];
    }
  }

  builder.build().tiles
}
