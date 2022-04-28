use std::collections::HashMap;

use raylib::color::Color;
use raylib::math::Vector2;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle, Texture2D};
use raylib::texture::RaylibTexture2D;
use serde::Deserialize;

/// A Deserializer for `EntityType`.
/// # Constraints
/// - `actions` must contain names of functions defined in the same mod.
#[derive(Deserialize)]
pub struct EntityTypeJSON {
    pub name: String,
    pub asset: String,
    pub actions: Vec<String>,
    pub states: Vec<EntityStateJSON>,
}

/// A Deserializer for the objects in `EntityTypeJSON`'s `state` property.
/// # Constraints
/// - the keys in `overrides` must correspond to properties of the corresponding `EntityTypeJSON`
#[derive(Deserialize)]
pub struct EntityStateJSON {
    pub name: String,
    pub overrides: HashMap<String, String>,
}

/// A descriptor for an entity.
/// # Constraints
/// 1. `label` must be unique
/// 2. `texture` must point to a file within a folder under the `assets` folder
pub struct EntityType {
    pub label: String,
    pub texture: Texture2D,
}

/// A single instance of an entity type.
#[repr(C)]
pub struct Entity<'et> {
    pub entity_type: &'et EntityType,
    pub translation: Vector2,
    pub rotation: f32,
    pub scale: f32,
}
impl<'et> Entity<'et> {
    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> anyhow::Result<()> {
        let translation = (
            self.translation.x - (self.entity_type.texture.width() / 2) as f32,
            self.translation.y - (self.entity_type.texture.height() / 2) as f32,
        );
        draw_handle.draw_texture_ex(
            &self.entity_type.texture,
            Vector2::from(translation),
            self.rotation,
            self.scale,
            Color::WHITE,
        );

        Ok(())
    }
}
