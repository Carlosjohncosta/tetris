mod tetris;
use std::env;

use nannou::color;
use nannou::prelude::*;
use tetris::*;

const BLOCK_SIZE: u32 = 25;
const GAME_WIDTH: u32 = 10;
const GAME_HEIGHT: u32 = 22;

struct Model {
    pause: bool,
    game_state: GameState,
}

fn model(_app: &App) -> Model {
    Model {
        pause: false,
        game_state: GameState::new(GAME_WIDTH, GAME_HEIGHT),
    }
}

fn draw_block(win: Rect, draw: &Draw, point: &Point, color: &Rgb<u8>) {
    let block_size_f = BLOCK_SIZE as f32;
    let Point { x, y } = point;
    let square = Rect::from_w_h(block_size_f, block_size_f)
        .bottom_left_of(win)
        .shift_x(*x as f32 * block_size_f)
        .shift_y(*y as f32 * block_size_f);
    draw.rect().xy(square.xy()).wh(square.wh()).color(*color);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);
    let piece = model.game_state.get_current_piece();

    for point in piece.get_block_positions().iter() {
        draw_block(win, &draw, point, &piece.color);
    }

    let board = model.game_state.get_board();
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.get_row().iter().enumerate() {
            if let Some(color) = cell {
                draw_block(win, &draw, &Point::new(x as f32, y as f32), color)
            }
        }
    }

    if let Some(break_frame) = model.game_state.get_break_frame() {
        let color = if break_frame % 2 == 0 {
            color::BLACK
        } else {
            color::WHITE
        };
        for (y, row) in model.game_state.get_board().iter().enumerate() {
            let block_size_f = BLOCK_SIZE as f32;
            if row.is_full() {
                let square = Rect::from_w_h(block_size_f * GAME_WIDTH as f32, block_size_f)
                    .bottom_left_of(win)
                    .shift_x(0.0 * block_size_f)
                    .shift_y(y as f32 * block_size_f);
                draw.rect().xy(square.xy()).wh(square.wh()).color(color);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.pause {
        return;
    }
    model.game_state.next_frame();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        id: _,
        simple: Some(WindowEvent::KeyPressed(key)),
    } = event
    {
        match key {
            Key::Left => _ = model.game_state.move_current_piece(Axis::X, -1.0),
            Key::Right => _ = model.game_state.move_current_piece(Axis::X, 1.0),
            Key::Up => model.game_state.rotate_current_piece(Direction::Clockwise),
            Key::P => model.pause = !model.pause,
            _ => {}
        };
    }
}

fn main() {
    println!("{:?}", env::current_dir().unwrap());
    nannou::app(model)
        .size(GAME_WIDTH * BLOCK_SIZE, GAME_HEIGHT * BLOCK_SIZE)
        .simple_window(view)
        .update(update)
        .event(event)
        .run()
}