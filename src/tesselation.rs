use omega8::Point8;
use omega8::get_octagon;
use omega8::get_square;
use point2d::Point2d;
use point2d::get_point2d_list;

pub type Tile = Vec<Point8>; // use in the model
pub type Tile2d = Vec<Point2d>; // use when outputting to 2d coords

pub fn get_model_point8() -> Vec<Tile> {
  let mut res: Vec<Tile> = Vec::new();

  // construct an initial central octagon
  let centre_octagon = get_octagon(Point8{x: [-1, 0], y: [-1, -1]}, 0);

  // construct a set of squares surrounding the central octagon, on diagonals
  for [point_index, initial_direction] in [[1, 7], [3, 1], [5, 3], [7, 5]] {
    let anchor_point: Point8 = centre_octagon[point_index].clone();
    let adjacent_square = get_square(anchor_point, initial_direction);
    res.push(adjacent_square);
  }

  // construct a set of octagons surrounding the central octagon
  for [point_index, initial_direction] in [[2, 7], [4, 1], [6, 3], [0, 5]] {
    let anchor_point: Point8 = centre_octagon[point_index].clone();
    let adjacent_octagon = get_octagon(anchor_point, initial_direction);
    res.push(adjacent_octagon);
  }

  // now we can push the central octagon as we don't need to reference it anymore
  res.push(centre_octagon);

  // that's it
  res
}

pub fn get_model() -> Vec<Tile2d> {
  return get_model_point8().iter().map(get_point2d_list).collect();
}
