extern crate piston_window;

use piston_window::*;

// Define game constants
const GAME_WIDTH: i32 = 25;
const GAME_HEIGHT: i32 = 25;
const BLOCK_SIZE: f64 = 20.0; // Size of each block (snake segment, food) in pixels

fn main() {
    // Calculate window dimensions
    let window_width = GAME_WIDTH as f64 * BLOCK_SIZE;
    let window_height = GAME_HEIGHT as f64 * BLOCK_SIZE;

    let mut window: PistonWindow = WindowSettings::new(
            "Snake Game",
            [window_width, window_height],
        )
        .exit_on_esc(true) // Close on ESC key press
        .build()
        .unwrap();

    // Game loop: This will run forever until window is closed
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1, 0.1, 0.1, 1.0], graphics); // Dark grey background
            // Drawing code will go here later
        });
    }
}