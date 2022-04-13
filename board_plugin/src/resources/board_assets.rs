use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;

#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct BoardAssets {
    /// Label
    pub label: String,
    ///
    pub board_material: SpriteMaterial,
    ///
    pub tile_material: SpriteMaterial,
    ///
    pub blue_pawn_material: SpriteMaterial,
    ///
    pub blue_king_material: SpriteMaterial,
    ///
    pub red_pawn_material: SpriteMaterial,
    ///
    pub red_king_material: SpriteMaterial,
    /// Material for the center point in the Card board
    pub deck_card_center_material: SpriteMaterial,
    ///
    pub deck_card_allowed_move_material: SpriteMaterial,
    ///
    pub selected_card_material: SpriteMaterial,
    ///
    pub selected_piece_material: SpriteMaterial,
    ///
    pub allowed_move_tile_material: SpriteMaterial,
    ///
    pub guide_text_size: f32,
    ///
    pub font: Handle<Font>,
}
