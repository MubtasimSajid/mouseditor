use macroquad::prelude::*;

const MOVE_VAL: f32 = 10.0;
const RADIUS: f32 = 12.0;
const THICKNESS: f32 = 2.5;

fn draw_keyboard() {
    let y_step = 100.0;

    for i in 0..3 {
        draw_line(
            screen_width() / 2.0,
            (screen_height() / 2.0) - (y_step / 2.0) - (y_step * (i as f32)),
            screen_width(),
            (screen_height() / 2.0) - (y_step / 2.0) - (y_step * (i as f32)),
            THICKNESS,
            GRAY,
        );

        draw_line(
            screen_width() / 2.0,
            (screen_height() / 2.0) + (y_step / 2.0) + (y_step * (i as f32)),
            screen_width(),
            (screen_height() / 2.0) + (y_step / 2.0) + (y_step * (i as f32)),
            THICKNESS,
            GRAY,
        );
    }

    for i in 1..=15 {
        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - (y_step * 2.0) - (y_step / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - y_step - (y_step / 2.0),
            THICKNESS,
            GRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - y_step - (y_step / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - (y_step / 2.0),
            THICKNESS,
            GRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - (y_step / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + (y_step / 2.0),
            THICKNESS,
            GRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + y_step + (y_step / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + (y_step / 2.0),
            THICKNESS,
            GRAY,
        );
    }

    draw_line(
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 - (y_step / 2.0),
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 + (y_step / 2.0),
        THICKNESS,
        LIGHTGRAY,
    );

    draw_line(
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 + y_step + (y_step / 2.0),
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 + (y_step / 2.0),
        THICKNESS,
        LIGHTGRAY,
    );

    draw_line(
        screen_width() / 2.0,
        screen_height() / 2.0 + (y_step / 2.0),
        screen_width(),
        screen_height() / 2.0 + (y_step / 2.0),
        THICKNESS,
        GRAY,
    );
}

#[macroquad::main("mouseditor")]
async fn main() {
    let mut x_pos = screen_width() / 2.0;
    let mut y_pos = screen_height() / 2.0;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
            x_pos = f32::min(screen_width() - RADIUS, x_pos + MOVE_VAL);
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
            x_pos = f32::max(RADIUS, x_pos - MOVE_VAL);
        } else if is_key_down(KeyCode::Up) || is_key_down(KeyCode::K) {
            y_pos = f32::max(RADIUS, y_pos - MOVE_VAL);
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::J) {
            y_pos = f32::min(screen_height() - RADIUS, y_pos + MOVE_VAL);
        } else if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            break;
        } else if is_key_down(KeyCode::Enter) {
            // Take in keypress
        }

        draw_line(
            screen_width() / 2.0,
            0.0,
            screen_width() / 2.0,
            screen_height(),
            5.0,
            BLUE,
        );
        draw_keyboard();
        draw_circle(x_pos, y_pos, RADIUS, RED);

        next_frame().await;
    }
}
