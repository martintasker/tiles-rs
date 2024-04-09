pub struct Point2d(pub f64, pub f64);

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

pub fn get_model() -> Vec<Tile> {
  return vec![
    get_square(-0.5, -0.5),
    get_square(0.5, -0.5)
  ]
}