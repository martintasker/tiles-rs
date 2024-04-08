use std::fs::File;
use std::io::prelude::*;

const SVG_TOP: &str = r#"
<svg viewBox="-5 -5 10 10" xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
  <g transform="scale(1 -1)">
"#;
const SVG_TAIL: &str = r#"
  </g>
</svg>
"#;

struct Point2d(f64, f64);

const POLYGON_TOP: &str ="    <polygon points=\"";
const POLYGON_TAIL: &str = "\" stroke-width=\"0.2%\" stroke=\"black\" fill=\"lightgrey\" />";

fn svg_polygon(points: Vec<Point2d>) -> String {
  let mut res: String = String::from(POLYGON_TOP);
  let points_str: Vec<String> = points.iter().map(|p| format!("{},{}", p.0, p.1)).collect();
  res.push_str(&points_str.join(" "));
  res.push_str(POLYGON_TAIL);
  res
}

fn main() -> std::io::Result<()> {
  let mut file = File::create("out.svg")?;
  file.write_all(SVG_TOP.as_bytes())?;
  let mut points = Vec::new();
  points.push(Point2d(0.5, 0.5));
  points.push(Point2d(-0.5, 0.5));
  points.push(Point2d(-0.5, -0.5));
  points.push(Point2d(0.5, -0.5));
  file.write_all(svg_polygon(points).as_bytes())?;
  file.write_all(SVG_TAIL.as_bytes())?;
  Ok(())
}
