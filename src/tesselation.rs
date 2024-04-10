use omega8::Point8;
use omega8::get_octagon;
use omega8::get_square;

pub type Tile = Vec<Point8>; // use in the model

pub fn get_model() -> Vec<Tile> {
  let mut res: Vec<Tile> = Vec::new();

  // construct an initial central octagon
  let centre_octagon = get_octagon(Point8{x: [-1, 0], y: [-1, -1]}, 0);
  res.push(centre_octagon.clone());

  // construct a set of squares surrounding the central octagon, on diagonals
  for [point_index, initial_direction] in [[1, 7], [3, 1], [5, 3], [7, 5]] {
    let anchor_point: Point8 = centre_octagon[point_index];
    let adjacent_square = get_square(anchor_point, initial_direction);
    res.push(adjacent_square);
  }

  // construct a set of octagons surrounding the central octagon
  for [point_index, initial_direction] in [[2, 7], [4, 1], [6, 3], [0, 5]] {
    let anchor_point: Point8 = centre_octagon[point_index];
    let adjacent_octagon = get_octagon(anchor_point, initial_direction);
    res.push(adjacent_octagon);
  }

  // that's it
  res
}
