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

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [window_width, window_height])
        .exit_on_esc(true) // Close on ESC key press
        .build()
        .unwrap();

    // Game loop: This will run forever until window is closed
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1, 0.1, 0.1, 1.0], graphics); // Dark grey background
            // Drawing code goes here
            draw(context, graphics);
        });
    }
}

/// Renders the snake to the screen.
fn draw(context: Context, graphics: &mut G2d) {
    // Draw head of snake
    let head = (12, 12); // Head position
    rectangle(
        [0.1, 0.8, 0.1, 1.0], // Bright green for head
        [
            head.0 as f64 * BLOCK_SIZE,
            head.1 as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        context.transform,
        graphics,
    );
    // Draw food
    let food = (8, 8); // Food position
    rectangle(
        [0.8, 0.1, 0.1, 1.0], // Red color
        [
            food.0 as f64 * BLOCK_SIZE,
            food.1 as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        context.transform,
        graphics,
    );
}
