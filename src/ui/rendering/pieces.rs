use crate::board::color::Color;
use crate::board::piece::Piece;
use crate::ui::constants;
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D};

pub struct Textures {
    white_pawn: Texture2D,
    black_pawn: Texture2D,
    white_knight: Texture2D,
    black_knight: Texture2D,
    white_bishop: Texture2D,
    black_bishop: Texture2D,
    white_rook: Texture2D,
    black_rook: Texture2D,
    white_queen: Texture2D,
    black_queen: Texture2D,
    white_king: Texture2D,
    black_king: Texture2D,
}

impl Textures {
    /// loads the textures
    pub async fn new() -> Result<Self, macroquad::Error> {
        Ok(Textures {
            white_pawn: load_texture("assets/piece1.png").await?,
            white_knight: load_texture("assets/piece2.png").await?,
            white_bishop: load_texture("assets/piece4.png").await?,
            white_rook: load_texture("assets/piece3.png").await?,
            white_queen: load_texture("assets/piece5.png").await?,
            white_king: load_texture("assets/piece6.png").await?,
            black_pawn: load_texture("assets/piece7.png").await?,
            black_knight: load_texture("assets/piece8.png").await?,
            black_bishop: load_texture("assets/piece10.png").await?,
            black_rook: load_texture("assets/piece9.png").await?,
            black_queen: load_texture("assets/piece11.png").await?,
            black_king: load_texture("assets/piece12.png").await?,
        })
    }

    pub fn get_texture(&self, piece: &Piece) -> &Texture2D {
        match (piece) {
            Piece::Pawn { color, .. } => match color {
                Color::Black => &self.black_pawn,
                Color::White => &self.white_pawn
            },
            Piece::Knight { color, .. } => match color {
                Color::Black => &self.black_knight,
                Color::White => &self.white_knight
            },
            Piece::Bishop { color, .. } => match color {
                Color::Black => &self.black_bishop,
                Color::White => &self.white_bishop
            },
            Piece::Rook { color, .. } => match color {
                Color::Black => &self.black_rook,
                Color::White => &self.white_rook
            },
            Piece::Queen { color, .. } => match color {
                Color::Black => &self.black_queen,
                Color::White => &self.white_queen
            },
            Piece::King { color, .. } => match color {
                Color::Black => &self.black_king,
                Color::White => &self.white_king
            }
        }
    }
}

pub fn draw_piece(textures: &Textures, x: i32, y: i32, piece: Piece) {
    draw_texture_ex(
        textures.get_texture(&piece),
        (x * constants::board::SQUARE_SIZE) as f32,
        (y * constants::board::SQUARE_SIZE) as f32,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from((
                constants::board::SQUARE_SIZE as f32,
                constants::board::SQUARE_SIZE as f32
            ))),
            ..Default::default()
        }
    );

    // draw_rectangle(
    //     constants::board::SQUARE_SIZE as f32,
    //     constants::board::SQUARE_SIZE as f32,
    //     macroquad::color::Color::from_rgba(255, 255, 255, 255),
    // )
}
