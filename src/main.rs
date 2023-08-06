mod board;
mod game;
mod logic;

use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::board::*;
use crate::game::*;
use crate::logic::*;

use nannou::prelude::*;

struct Model {
    game: Game,
    last_move_time: std::time::Instant,
    textures: HashMap<&'static str, wgpu::Texture>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let assets_path: PathBuf = app.assets_path().unwrap();
    let img_path: PathBuf = assets_path.join("img");

    Model {
        game: Game::default(),
        last_move_time: std::time::Instant::now(),
        textures: HashMap::from([
            (
                "square_dark",
                wgpu::Texture::from_path(app, img_path.join("square_dark.png")).unwrap(),
            ),
            (
                "square_light",
                wgpu::Texture::from_path(app, img_path.join("square_light.png")).unwrap(),
            ),
            (
                "pawn_white",
                wgpu::Texture::from_path(app, img_path.join("pawn_white.png")).unwrap(),
            ),
            (
                "rook_white",
                wgpu::Texture::from_path(app, img_path.join("rook_white.png")).unwrap(),
            ),
            (
                "knight_white",
                wgpu::Texture::from_path(app, img_path.join("knight_white.png")).unwrap(),
            ),
            (
                "bishop_white",
                wgpu::Texture::from_path(app, img_path.join("bishop_white.png")).unwrap(),
            ),
            (
                "king_white",
                wgpu::Texture::from_path(app, img_path.join("king_white.png")).unwrap(),
            ),
            (
                "queen_white",
                wgpu::Texture::from_path(app, img_path.join("queen_white.png")).unwrap(),
            ),
            (
                "pawn_black",
                wgpu::Texture::from_path(app, img_path.join("pawn_black.png")).unwrap(),
            ),
            (
                "rook_black",
                wgpu::Texture::from_path(app, img_path.join("rook_black.png")).unwrap(),
            ),
            (
                "knight_black",
                wgpu::Texture::from_path(app, img_path.join("knight_black.png")).unwrap(),
            ),
            (
                "bishop_black",
                wgpu::Texture::from_path(app, img_path.join("bishop_black.png")).unwrap(),
            ),
            (
                "king_black",
                wgpu::Texture::from_path(app, img_path.join("king_black.png")).unwrap(),
            ),
            (
                "queen_black",
                wgpu::Texture::from_path(app, img_path.join("queen_black.png")).unwrap(),
            ),
        ]),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.last_move_time.elapsed().as_millis() < 1000 {
        return;
    }

    model.last_move_time = std::time::Instant::now();

    let game = model.game;

    let mut all_valid_moves: Vec<Move> = vec![];

    // Clone the squares to release the borrow on game
    let squares = game.board().squares().to_vec();

    for (index, square) in squares.iter().enumerate() {
        if let Some(piece) = square {
            println!("Piece: {:?}", piece);

            let valid_moves = logic::get_valid_moves(game.board(), index);

            if valid_moves.len() > 0 {
                let random_move: Option<&usize> = valid_moves.choose(&mut rand::thread_rng());

                if let Some(valid_move) = random_move {
                    all_valid_moves.push({
                        Move::new(index, *valid_move)
                    });
                }
            }
        }
    }

    if all_valid_moves.len() > 0 {
        let random_move: Option<&Move> = all_valid_moves.choose(&mut rand::thread_rng());

        if let Some(valid_move) = random_move {
            model.game.move_piece(valid_move.from(), valid_move.to());
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win: Rect = app.window_rect();

    let draw: Draw = app.draw();

    draw_squares(&draw, &model, win);
    draw_pieces(&draw, &model, win);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_pieces(draw: &Draw, model: &Model, win: Rect) {
    let square_size = win.w() / 8.0;

    let squares = model.game.board().squares();

    for row in 0..8 {
        for col in 0..8 {
            let x: f32 = col as f32 * square_size - win.w() / 2.0 + square_size / 2.0;
            let y: f32 = row as f32 * square_size - win.h() / 2.0 + square_size / 2.0;

            // Calculate the index into the one-dimensional squares slice
            let index: usize = row * 8 + col;

            if let Some(piece) = &squares[index] {
                let texture = match piece.piece_type() {
                    PieceType::Pawn => {
                        if *piece.colour() == Colour::White {
                            &model.textures["pawn_white"]
                        } else {
                            &model.textures["pawn_black"]
                        }
                    }
                    PieceType::Rook => {
                        if *piece.colour() == Colour::White {
                            &model.textures["rook_white"]
                        } else {
                            &model.textures["rook_black"]
                        }
                    }
                    PieceType::Knight => {
                        if *piece.colour() == Colour::White {
                            &model.textures["knight_white"]
                        } else {
                            &model.textures["knight_black"]
                        }
                    }
                    PieceType::Bishop => {
                        if *piece.colour() == Colour::White {
                            &model.textures["bishop_white"]
                        } else {
                            &model.textures["bishop_black"]
                        }
                    }
                    PieceType::King => {
                        if *piece.colour() == Colour::White {
                            &model.textures["king_white"]
                        } else {
                            &model.textures["king_black"]
                        }
                    }
                    PieceType::Queen => {
                        if *piece.colour() == Colour::White {
                            &model.textures["queen_white"]
                        } else {
                            &model.textures["queen_black"]
                        }
                    }
                };

                draw.texture(texture).x_y(x, y);
            }
        }
    }
}

fn draw_squares(draw: &Draw, model: &Model, win: Rect) {
    let square_size = win.w() / 8.0;

    for row in 0..8 {
        for col in 0..8 {
            let x = col as f32 * square_size - win.w() / 2.0 + square_size / 2.0;
            let y = row as f32 * square_size - win.h() / 2.0 + square_size / 2.0;

            let texture = if (row + col) % 2 == 0 {
                &model.textures["square_dark"]
            } else {
                &model.textures["square_light"]
            };

            draw.texture(texture).x_y(x, y);
        }
    }
}
