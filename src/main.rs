pub mod tesselation;
pub mod svg_writer;

use svg_writer::SVGWriter;

fn main() -> std::io::Result<()> {
  let mut writer = SVGWriter::new("out.svg")?;
  let square = tesselation::get_square();
  writer.write_model(square)?;
  Ok(())
}
