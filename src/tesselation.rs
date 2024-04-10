use omega8::Point8;
use omega8::get_octagon;
use point2d::Point2d;
use point2d::get_point2d_list;

pub type Tile = Vec<Point2d>;

pub fn get_square(x0: f64, y0: f64) -> Tile {
  let square = vec![
    Point2d(x0, y0),
    Point2d(x0+1.0, y0),
    Point2d(x0+1.0, y0+1.0),
    Point2d(x0, y0+1.0)
  ];
  return square;
}

pub fn get_octagon_as_tile() -> Tile {
  let octagon = get_octagon(Point8{x: [-1, 0], y: [-1, -1]}, 0);
  return get_point2d_list(octagon);
}

pub fn get_model() -> Vec<Tile> {
  return vec![
    get_octagon_as_tile(),
    get_square(-0.5, -0.5),
    get_square(0.5, -0.5)
  ]
}
