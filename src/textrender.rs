use rusttype::{Font, Scale};
use rusttype::point;

use crate::framebuffer::Framebuffer;

pub struct TextRenderer<'a> {
    font: Font<'a>,
    scale: Scale,
}

impl<'a> TextRenderer<'a> {
    pub fn new(font_data: &'a [u8], scale: f32) -> Self {
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(scale);

        TextRenderer { font, scale }
    }

    pub fn render_text(&self, framebuffer: &mut Framebuffer, text: &str, x: f32, y: f32, color: u32) {
        let v_metrics = self.font.v_metrics(self.scale);

        let glyphs: Vec<_> = self.font.layout(text, self.scale, point(x, y + v_metrics.ascent)).collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let x = gx + bounding_box.min.x as u32;
                    let y = gy + bounding_box.min.y as u32;
                    if (x < framebuffer.width as u32) && (y < framebuffer.height as u32) && v > 0.5 {
                        framebuffer.set_current_color(color);
                        framebuffer.point(x as usize, y as usize);
                    }
                });
            }
        }
    }
}
