mod tesselation;
mod svg_writer;
mod xy_point;
mod xy_util;
mod omega_coords;
mod omega_space;
mod omega_shapes;
mod omega_tiles;
mod omega8;
mod model8;
mod omega12;
mod model12;
mod spectre_model;
mod shapes;

use svg_writer::SVGWriter;
use xy_point::XYPoint;
use xy_util::{get_xy_point_list_from12, get_xy_point_list_from8, is_all_inside};

use omega_shapes::get_square;
use omega_space::{OMEGA8_SPACE, OmegaSpacePoint};
use omega_tiles::{Tile, Tesselation};

fn main() -> std::io::Result<()> {
  {
    let mut writer = SVGWriter::new("out-new-square.svg")?;
    let square = get_square(OmegaSpacePoint::new(&OMEGA8_SPACE), 0);
    let tile0 = Tile::new(&square);
    let mut tesselation: Tesselation<8, 2> = Tesselation::new();
    tesselation.add(tile0);
    let new_square_model_xy = tesselation
      .get_tiles()
      .iter()
      .map(get_xy_list_from_tile)
      .filter(is_all_inside)
      .collect();
    writer.write_model(new_square_model_xy)?;
  }
  {
    let mut writer = SVGWriter::new("out-spectre.svg")?;
    let spectre_model = spectre_model::get();
    let spectre_model_xy = spectre_model.iter().map(get_xy_point_list_from12).filter(is_all_inside).collect();
    writer.write_model(spectre_model_xy)?;
  }
  {
    let mut writer = SVGWriter::new("out8.svg")?;
    let model8 = model8::get();
    let model8_xy = model8.iter().map(get_xy_point_list_from8).filter(is_all_inside).collect();
    writer.write_model(model8_xy)?;
  }
  {
    let mut writer = SVGWriter::new("out12.svg")?;
    let model12 = model12::get();
    let model12_xy = model12.iter().map(get_xy_point_list_from12).filter(is_all_inside).collect();
    writer.write_model(model12_xy)?;
  }
  Ok(())
}

fn get_xy_list_from_tile<'a>(tile: &Tile<'a, 8, 2>) -> Vec<XYPoint> {
  return tile
    .get_points()
    .iter()
    .map(get_xy_from_omega8)
  .collect();
}

fn get_xy_from_omega8<'a>(point: &OmegaSpacePoint<'a, 8, 2>) -> XYPoint {
  point.to_xy_point()
}
