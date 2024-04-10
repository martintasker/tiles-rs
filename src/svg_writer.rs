// there's no claim to beauty in here -- we'll come back to this
// attention for the moment is in producing the tesselation

use std::fs::File;
use std::io::prelude::*;

use point2d::{Point2d, Tile2d};

pub struct SVGWriter {
  file: File,
}

const SVG_TOP: &str = r#"
<svg viewBox="-20 -20 40 40" xmlns="http://www.w3.org/2000/svg" width="800" height="800">
  <g transform="scale(1 -1)">
  <rect x="-20" y="-20" width="40" height="40" fill="darkgrey" stroke-width="0" />
"#;
const SVG_TAIL: &str = r#"
  </g>
</svg>
"#;

const POLYGON_TOP: &str ="    <polygon points=\"";
const POLYGON_TAIL: &str = "\" stroke-width=\"0.2%\" stroke=\"black\" fill=\"lightgrey\" />\n";

fn svg_polygon(points: Vec<Point2d>) -> String {
  let mut res = String::from(POLYGON_TOP);
  let points_str: Vec<String> = points.iter().map(|p| format!("{},{}", p.0, p.1)).collect();
  res.push_str(&points_str.join(" "));
  res.push_str(POLYGON_TAIL);
  res
}

impl SVGWriter {
  pub fn new(filename: &str) -> std::io::Result<Self> {
    Ok(Self { file: File::create(filename)? })
  }

  pub fn write_model(&mut self, model: Vec<Tile2d>) -> std::io::Result<()> {
    self.file.write_all(SVG_TOP.as_bytes())?;
    for tile in model {
      self.file.write_all(svg_polygon(tile).as_bytes())?;
    }
    self.file.write_all(SVG_TAIL.as_bytes())?;
    Ok(())
  }
}
