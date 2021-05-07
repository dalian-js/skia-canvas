#![allow(clippy::unnecessary_wraps)]
use std::sync::{Mutex};
use neon::prelude::*;

#[macro_use]
extern crate lazy_static;

mod canvas;
mod context;
mod path;
mod image;
mod gradient;
mod pattern;
mod typography;
mod utils;

use context::api as ctx;
use typography::FontLibrary;

lazy_static! {
  pub static ref FONT_LIBRARY:Mutex<FontLibrary> = FontLibrary::shared();
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("Image_new", image::image_new)?;
  cx.export_function("Image_get_src", image::image_get_src)?;
  cx.export_function("Image_set_src", image::image_set_src)?;
  cx.export_function("Image_set_data", image::image_set_data)?;
  cx.export_function("Image_get_width", image::image_get_width)?;
  cx.export_function("Image_get_height", image::image_get_height)?;
  cx.export_function("Image_get_complete", image::image_get_complete)?;


  cx.export_function("Path2D_new", path::path2d_new)?;
  cx.export_function("Path2D_from_path2d", path::path2d_from_path2d)?;
  cx.export_function("Path2D_from_svg", path::path2d_from_svg)?;
  cx.export_function("Path2D_add_path", path::path2d_add_path)?;
  cx.export_function("Path2D_add_path_matrix", path::path2d_add_path_matrix)?;
  cx.export_function("Path2D_close_path", path::path2d_close_path)?;
  cx.export_function("Path2D_move_to", path::path2d_move_to)?;
  cx.export_function("Path2D_line_to", path::path2d_line_to)?;
  cx.export_function("Path2D_bezier_curve_to", path::path2d_bezier_curve_to)?;
  cx.export_function("Path2D_quadratic_curve_to", path::path2d_quadratic_curve_to)?;
  cx.export_function("Path2D_arc", path::path2d_arc)?;
  cx.export_function("Path2D_arc_to", path::path2d_arc_to)?;
  cx.export_function("Path2D_ellipse", path::path2d_ellipse)?;
  cx.export_function("Path2D_rect", path::path2d_rect)?;
  cx.export_function("Path2D_op", path::path2d_op)?;
  cx.export_function("Path2D_simplify", path::path2d_simplify)?;
  cx.export_function("Path2D_bounds", path::path2d_bounds)?;

  cx.export_function("CanvasGradient_linear", gradient::canvasgradient_linear)?;
  cx.export_function("CanvasGradient_radial", gradient::canvasgradient_radial)?;
  cx.export_function("CanvasGradient_add_color_stop", gradient::canvasgradient_add_color_stop)?;

  cx.export_function("CanvasPattern_from_image", pattern::canvaspattern_from_image)?;
  // cx.export_function("CanvasPattern_from_canvas", pattern::canvaspattern_from_canvas)?;
  cx.export_function("CanvasPattern_set_transform", pattern::canvaspattern_set_transform)?;

  cx.export_function("FontLibrary_get_families", typography::fontlibrary_get_families)?;
  cx.export_function("FontLibrary_has", typography::fontlibrary_has)?;
  cx.export_function("FontLibrary_family", typography::fontlibrary_family)?;
  cx.export_function("FontLibrary_add_family", typography::fontlibrary_add_family)?;

  cx.export_function("Canvas_new", canvas::canvas_new)?;
  cx.export_function("Canvas_get_width", canvas::canvas_get_width)?;
  cx.export_function("Canvas_get_height", canvas::canvas_get_height)?;
  cx.export_function("Canvas_set_width", canvas::canvas_set_width)?;
  cx.export_function("Canvas_set_height", canvas::canvas_set_height)?;
  cx.export_function("Canvas_save_as", canvas::canvas_save_as)?;
  cx.export_function("Canvas_to_buffer", canvas::canvas_to_buffer)?;

  cx.export_function("CanvasRenderingContext2D_new", ctx::new)?;
  cx.export_function("CanvasRenderingContext2D_save", ctx::save)?;
  cx.export_function("CanvasRenderingContext2D_restore", ctx::restore)?;
  cx.export_function("CanvasRenderingContext2D_clip", ctx::clip)?;
  cx.export_function("CanvasRenderingContext2D_transform", ctx::transform)?;
  cx.export_function("CanvasRenderingContext2D_translate", ctx::translate)?;
  cx.export_function("CanvasRenderingContext2D_scale", ctx::scale)?;
  cx.export_function("CanvasRenderingContext2D_rotate", ctx::rotate)?;
  cx.export_function("CanvasRenderingContext2D_resetTransform", ctx::resetTransform)?;
  cx.export_function("CanvasRenderingContext2D_get_currentTransform", ctx::get_currentTransform)?;
  cx.export_function("CanvasRenderingContext2D_set_currentTransform", ctx::set_currentTransform)?;
  cx.export_function("CanvasRenderingContext2D_beginPath", ctx::beginPath)?;
  cx.export_function("CanvasRenderingContext2D_rect", ctx::rect)?;
  cx.export_function("CanvasRenderingContext2D_arc", ctx::arc)?;
  cx.export_function("CanvasRenderingContext2D_ellipse", ctx::ellipse)?;
  cx.export_function("CanvasRenderingContext2D_moveTo", ctx::moveTo)?;
  cx.export_function("CanvasRenderingContext2D_lineTo", ctx::lineTo)?;
  cx.export_function("CanvasRenderingContext2D_arcTo", ctx::arcTo)?;
  cx.export_function("CanvasRenderingContext2D_bezierCurveTo", ctx::bezierCurveTo)?;
  cx.export_function("CanvasRenderingContext2D_quadraticCurveTo", ctx::quadraticCurveTo)?;
  cx.export_function("CanvasRenderingContext2D_closePath", ctx::closePath)?;
  cx.export_function("CanvasRenderingContext2D_isPointInPath", ctx::isPointInPath)?;
  cx.export_function("CanvasRenderingContext2D_isPointInStroke", ctx::isPointInStroke)?;
  cx.export_function("CanvasRenderingContext2D_fill", ctx::fill)?;
  cx.export_function("CanvasRenderingContext2D_stroke", ctx::stroke)?;
  cx.export_function("CanvasRenderingContext2D_fillRect", ctx::fillRect)?;
  cx.export_function("CanvasRenderingContext2D_strokeRect", ctx::strokeRect)?;
  cx.export_function("CanvasRenderingContext2D_clearRect", ctx::clearRect)?;
  cx.export_function("CanvasRenderingContext2D_get_fillStyle", ctx::get_fillStyle)?;
  cx.export_function("CanvasRenderingContext2D_set_fillStyle", ctx::set_fillStyle)?;
  cx.export_function("CanvasRenderingContext2D_get_strokeStyle", ctx::get_strokeStyle)?;
  cx.export_function("CanvasRenderingContext2D_set_strokeStyle", ctx::set_strokeStyle)?;
  cx.export_function("CanvasRenderingContext2D_getLineDash", ctx::getLineDash)?;
  cx.export_function("CanvasRenderingContext2D_setLineDash", ctx::setLineDash)?;
  cx.export_function("CanvasRenderingContext2D_get_lineCap", ctx::get_lineCap)?;
  cx.export_function("CanvasRenderingContext2D_set_lineCap", ctx::set_lineCap)?;
  cx.export_function("CanvasRenderingContext2D_get_lineDashOffset", ctx::get_lineDashOffset)?;
  cx.export_function("CanvasRenderingContext2D_set_lineDashOffset", ctx::set_lineDashOffset)?;
  cx.export_function("CanvasRenderingContext2D_get_lineJoin", ctx::get_lineJoin)?;
  cx.export_function("CanvasRenderingContext2D_set_lineJoin", ctx::set_lineJoin)?;
  cx.export_function("CanvasRenderingContext2D_get_lineWidth", ctx::get_lineWidth)?;
  cx.export_function("CanvasRenderingContext2D_set_lineWidth", ctx::set_lineWidth)?;
  cx.export_function("CanvasRenderingContext2D_get_miterLimit", ctx::get_miterLimit)?;
  cx.export_function("CanvasRenderingContext2D_drawImage", ctx::drawImage)?;
  cx.export_function("CanvasRenderingContext2D_getImageData", ctx::getImageData)?;
  cx.export_function("CanvasRenderingContext2D_putImageData", ctx::putImageData)?;
  cx.export_function("CanvasRenderingContext2D_get_imageSmoothingEnabled", ctx::get_imageSmoothingEnabled)?;
  cx.export_function("CanvasRenderingContext2D_set_imageSmoothingEnabled", ctx::set_imageSmoothingEnabled)?;
  cx.export_function("CanvasRenderingContext2D_get_imageSmoothingQuality", ctx::get_imageSmoothingQuality)?;
  cx.export_function("CanvasRenderingContext2D_set_imageSmoothingQuality", ctx::set_imageSmoothingQuality)?;
  cx.export_function("CanvasRenderingContext2D_fillText", ctx::fillText)?;
  cx.export_function("CanvasRenderingContext2D_strokeText", ctx::strokeText)?;
  cx.export_function("CanvasRenderingContext2D_measureText", ctx::measureText)?;
  cx.export_function("CanvasRenderingContext2D_get_font", ctx::get_font)?;
  cx.export_function("CanvasRenderingContext2D_set_font", ctx::set_font)?;
  cx.export_function("CanvasRenderingContext2D_get_textAlign", ctx::get_textAlign)?;
  cx.export_function("CanvasRenderingContext2D_set_textAlign", ctx::set_textAlign)?;
  cx.export_function("CanvasRenderingContext2D_get_textBaseline", ctx::get_textBaseline)?;
  cx.export_function("CanvasRenderingContext2D_set_textBaseline", ctx::set_textBaseline)?;
  cx.export_function("CanvasRenderingContext2D_get_direction", ctx::get_direction)?;
  cx.export_function("CanvasRenderingContext2D_set_direction", ctx::set_direction)?;
  cx.export_function("CanvasRenderingContext2D_get_fontVariant", ctx::get_fontVariant)?;
  cx.export_function("CanvasRenderingContext2D_set_fontVariant", ctx::set_fontVariant)?;
  cx.export_function("CanvasRenderingContext2D_get_textTracking", ctx::get_textTracking)?;
  cx.export_function("CanvasRenderingContext2D_set_textTracking", ctx::set_textTracking)?;
  cx.export_function("CanvasRenderingContext2D_get_textWrap", ctx::get_textWrap)?;
  cx.export_function("CanvasRenderingContext2D_set_textWrap", ctx::set_textWrap)?;
  cx.export_function("CanvasRenderingContext2D_get_globalAlpha", ctx::get_globalAlpha)?;
  cx.export_function("CanvasRenderingContext2D_set_globalAlpha", ctx::set_globalAlpha)?;
  cx.export_function("CanvasRenderingContext2D_get_globalCompositeOperation", ctx::get_globalCompositeOperation)?;
  cx.export_function("CanvasRenderingContext2D_set_globalCompositeOperation", ctx::set_globalCompositeOperation)?;
  cx.export_function("CanvasRenderingContext2D_get_filter", ctx::get_filter)?;
  cx.export_function("CanvasRenderingContext2D_set_filter", ctx::set_filter)?;
  cx.export_function("CanvasRenderingContext2D_get_shadowBlur", ctx::get_shadowBlur)?;
  cx.export_function("CanvasRenderingContext2D_set_shadowBlur", ctx::set_shadowBlur)?;
  cx.export_function("CanvasRenderingContext2D_get_shadowColor", ctx::get_shadowColor)?;
  cx.export_function("CanvasRenderingContext2D_set_shadowColor", ctx::set_shadowColor)?;
  cx.export_function("CanvasRenderingContext2D_get_shadowOffsetX", ctx::get_shadowOffsetX)?;
  cx.export_function("CanvasRenderingContext2D_get_shadowOffsetY", ctx::get_shadowOffsetY)?;
  cx.export_function("CanvasRenderingContext2D_set_shadowOffsetX", ctx::set_shadowOffsetX)?;
  cx.export_function("CanvasRenderingContext2D_set_shadowOffsetY", ctx::set_shadowOffsetY)?;

  Ok(())
}