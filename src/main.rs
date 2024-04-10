mod tesselation;
mod svg_writer;
mod point2d;
mod omega8;

use svg_writer::SVGWriter;
use point2d::get_point2d_list;

fn main() -> std::io::Result<()> {
  let mut writer = SVGWriter::new("out.svg")?;
  let model = tesselation::get_model();
  let model_point2d = model.iter().map(get_point2d_list).collect();
  writer.write_model(model_point2d)?;
  Ok(())
}
