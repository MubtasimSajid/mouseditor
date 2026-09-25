use macroquad::prelude::*;

const MOVE_VAL: f32 = 6.0;
const RADIUS: f32 = 12.0;

fn draw_keyboard() {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let step = 40.0;
    let mut x = 0.0;
    let mut y = 0.0;

    while x < screen_w {
        draw_line(x, 0.0, x, screen_h, 1.0, GRAY);
        x += step;
    }

    while y < screen_h {
        draw_line(0.0, y, screen_w, y, 1.0, GRAY);
        y += step;
    }
}

#[macroquad::main("mouseditor")]
async fn main() {
    let mut x_pos = 200.0;
    let mut y_pos = 100.0;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) {
            x_pos = f32::min(screen_width() - RADIUS, x_pos + MOVE_VAL);
        } else if is_key_down(KeyCode::Left) {
            x_pos = f32::max(RADIUS, x_pos - MOVE_VAL);
        } else if is_key_down(KeyCode::Up) {
            y_pos = f32::max(RADIUS, y_pos - MOVE_VAL);
        } else if is_key_down(KeyCode::Down) {
            y_pos = f32::min(screen_height() - RADIUS, y_pos + MOVE_VAL);
        } else if is_key_down(KeyCode::Escape) {
            break;
        } else if is_key_down(KeyCode::Enter) {
            // Take in keypress
        }

        draw_keyboard();
        draw_circle(x_pos, y_pos, RADIUS, RED);

        next_frame().await;
    }
}
