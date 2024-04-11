mod tesselation;
mod svg_writer;
mod point2d;
mod omega8;
mod model8;
mod omega12;
mod model12;
mod shapes;

use svg_writer::SVGWriter;
use point2d::{get_point2d_list_from12, get_point2d_list_from8, is_all_inside};

fn main() -> std::io::Result<()> {
  let mut writer = SVGWriter::new("out8.svg")?;
  let model8 = model8::get();
  let model8_point2d = model8.iter().map(get_point2d_list_from8).filter(is_all_inside).collect();
  writer.write_model(model8_point2d)?;

  writer = SVGWriter::new("out12.svg")?;
  let model12 = model12::get();
  let model12_point2d = model12.iter().map(get_point2d_list_from12).filter(is_all_inside).collect();
  writer.write_model(model12_point2d)?;

  Ok(())
}
