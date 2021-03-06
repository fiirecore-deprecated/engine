use macroquad::prelude::{DrawTextureParams, FilterMode, Texture2D};
use std::rc::Rc;

use crate::{
    context::Context,
    math::{Rectangle, Vec2},
};

use super::{Color, Image};

#[derive(Debug, Clone, PartialEq)]
pub struct Texture(Rc<TextureData>);

impl Texture {
    pub(crate) fn crate_new(data: &[u8]) -> Result<Self, image::ImageError> {
        let image = image::load_from_memory_with_format(data, image::ImageFormat::Png)?.to_rgba8();
        Ok(Self::crate_from_image(&image))
    }

    pub(crate) fn crate_from_image(image: &image::RgbaImage) -> Self {
        let tex = Texture2D::from_rgba8(image.width() as _, image.height() as _, image.as_raw());
        tex.set_filter(FilterMode::Nearest);
        Self(Rc::new(TextureData(tex)))
    }

    #[allow(unused_variables)]
    pub fn new(ctx: &mut Context, data: &[u8]) -> Result<Self, image::ImageError> {
        Self::crate_new(data)
    }

    #[allow(unused_variables)]
    pub fn from_image(ctx: &mut Context, image: &Image) -> Self {
        Self::crate_from_image(&image.0)
    }

    #[allow(unused_variables)]
    pub fn draw(&self, ctx: &mut Context, x: f32, y: f32, params: DrawParams) {
        self.crate_draw(x, y, params)
    }

    pub(crate) fn crate_draw(&self, x: f32, y: f32, params: DrawParams) {
        let (color, params) = params.init();
        macroquad::prelude::draw_texture_ex(**self.0, x, y, color, params);
    }

    pub fn width(&self) -> f32 {
        self.0.width()
    }

    pub fn height(&self) -> f32 {
        self.0.height()
    }

    pub fn set_filter(&self, filter: FilterMode) {
        self.0.set_filter(filter)
    }

    pub fn data(&self) -> &TextureData {
        &self.0
    }

    // pub fn try_draw(self: Option<&Self>, ctx: &mut Context, x: f32, y: f32, params: DrawParams) {}
}

#[derive(Debug, PartialEq)]
pub struct TextureData(Texture2D);

impl core::ops::Deref for TextureData {
    type Target = Texture2D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for TextureData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        self.0.delete()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub color: Color,

    /// Part of texture to draw. If None - draw the whole texture.
    /// Good use example: drawing an image from texture atlas.
    /// Is None by default
    pub source: Option<Rectangle>,

    pub dest_size: Option<Vec2>,

    /// Rotation in radians
    pub rotation: f32,

    /// Mirror on the X axis
    pub flip_x: bool,

    /// Mirror on the Y axis
    pub flip_y: bool,

    pub origin: Option<Vec2>,
}

impl DrawParams {
    pub fn color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn source(source: Rectangle) -> Self {
        Self {
            source: Some(source),
            ..Default::default()
        }
    }

    pub(crate) fn init(self) -> (macroquad::prelude::Color, DrawTextureParams) {
        (
            self.color,
            DrawTextureParams {
                dest_size: self.dest_size,
                source: self.source,
                rotation: self.rotation,
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                pivot: self.origin,
            },
        )
    }
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            dest_size: None,
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            origin: None,
        }
    }
}
