use macroquad::prelude::*;

const MOVE_VAL: f32 = 10.0;
const RADIUS: f32 = 12.0;
const THICKNESS: f32 = 2.5;
const Y_STEP: f32 = 100.0;

fn draw_keyboard() {
    for i in 0..3 {
        draw_line(
            screen_width() / 2.0,
            (screen_height() / 2.0) - (Y_STEP / 2.0) - (Y_STEP * (i as f32)),
            screen_width(),
            (screen_height() / 2.0) - (Y_STEP / 2.0) - (Y_STEP * (i as f32)),
            THICKNESS,
            DARKGRAY,
        );

        draw_line(
            screen_width() / 2.0,
            (screen_height() / 2.0) + (Y_STEP / 2.0) + (Y_STEP * (i as f32)),
            screen_width(),
            (screen_height() / 2.0) + (Y_STEP / 2.0) + (Y_STEP * (i as f32)),
            THICKNESS,
            DARKGRAY,
        );
    }

    for i in 1..=15 {
        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - (Y_STEP * 2.0) - (Y_STEP / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - Y_STEP - (Y_STEP / 2.0),
            THICKNESS,
            DARKGRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - Y_STEP - (Y_STEP / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - (Y_STEP / 2.0),
            THICKNESS,
            DARKGRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 - (Y_STEP / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + (Y_STEP / 2.0),
            THICKNESS,
            DARKGRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + Y_STEP + (Y_STEP / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + (Y_STEP / 2.0),
            THICKNESS,
            DARKGRAY,
        );

        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + Y_STEP + (Y_STEP / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + (Y_STEP * 2.0) + (Y_STEP / 2.0),
            THICKNESS,
            DARKGRAY,
        );
    }

    draw_line(
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 - (Y_STEP / 2.0),
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 + (Y_STEP / 2.0),
        THICKNESS,
        LIGHTGRAY,
    );

    draw_line(
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 + Y_STEP + (Y_STEP / 2.0),
        (screen_width() / 2.0) + (screen_width() * 13.0 / 30.0),
        screen_height() / 2.0 + (Y_STEP / 2.0),
        THICKNESS,
        LIGHTGRAY,
    );

    draw_line(
        screen_width() / 2.0,
        screen_height() / 2.0 + (Y_STEP / 2.0),
        screen_width(),
        screen_height() / 2.0 + (Y_STEP / 2.0),
        THICKNESS,
        DARKGRAY,
    );

    for i in 4..=7 {
        draw_line(
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + Y_STEP + (Y_STEP / 2.0),
            (screen_width() / 2.0) + (screen_width() * (i as f32) / 30.0),
            screen_height() / 2.0 + (Y_STEP * 2.0) + (Y_STEP / 2.0),
            THICKNESS,
            LIGHTGRAY,
        );
    }
}

fn draw_labels() {
    let rows = [
        [
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "+", "B", "H",
        ],
        [
            "T", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\", "E",
        ],
        [
            "C", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "E", " ", "U",
        ],
        [
            "S", "z", "x", "c", "v", "b", "n", "m", ",", ".", ".", "/", "S", " ", "D",
        ],
        [
            "C", "S", "A", " ", " ", " ", " ", " ", "D", "A", "C", "R", "D", "U", "L",
        ],
    ];

    for i in 1..=15 {
        draw_text(
            rows[0][i - 1],
            screen_width() / 2.0
                + (screen_width() / 60.0)
                + (screen_width() * ((i - 1) as f32) / 30.0),
            screen_height() / 2.0 - (Y_STEP * 2.0),
            35.0,
            BLACK,
        );

        draw_text(
            rows[1][i - 1],
            screen_width() / 2.0
                + (screen_width() / 60.0)
                + (screen_width() * ((i - 1) as f32) / 30.0),
            screen_height() / 2.0 - Y_STEP,
            35.0,
            BLACK,
        );

        draw_text(
            rows[2][i - 1],
            screen_width() / 2.0
                + (screen_width() / 60.0)
                + (screen_width() * ((i - 1) as f32) / 30.0),
            screen_height() / 2.0,
            35.0,
            BLACK,
        );

        draw_text(
            rows[3][i - 1],
            screen_width() / 2.0
                + (screen_width() / 60.0)
                + (screen_width() * ((i - 1) as f32) / 30.0),
            screen_height() / 2.0 + Y_STEP,
            35.0,
            BLACK,
        );

        draw_text(
            rows[4][i - 1],
            screen_width() / 2.0
                + (screen_width() / 60.0)
                + (screen_width() * ((i - 1) as f32) / 30.0),
            screen_height() / 2.0 + (Y_STEP * 2.0),
            35.0,
            BLACK,
        );
    }
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
        draw_labels();
        draw_circle(x_pos, y_pos, RADIUS, RED);

        next_frame().await;
    }
}
