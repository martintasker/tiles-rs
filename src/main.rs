mod tesselation;
mod svg_writer;
mod omega8;

use svg_writer::SVGWriter;

fn main() -> std::io::Result<()> {
  let mut writer = SVGWriter::new("out.svg")?;
  let model= tesselation::get_model();
  writer.write_model(model)?;
  Ok(())
}
