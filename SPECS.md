# Snake Game written in Rust

## Here's the Plan

First, what's the bare minimum amount of work that I can put in and still
call this project done? I need to sketch a specification...

### Minimum Viable Product

These are the basic feature that need to be implemented for this game
to be done:

- Snake moves continuously.
- Player controls snake direction with arrow keys.
- Food appears randomly on the grid.
- Snake grows when it eats food.
- Score increases when food is eaten.
- Game ends if the snake hits a wall.
- Game ends if the snake hits itself.
- Player and Food must be visually distinct (different colors).

### Other Potential Features (for later? maybe..)

Let's move the goalsposts a little bit farther! Those are _Stretch Goals_,
they're nice to have but I will set them off until I have a version that
meets the basic spec, here's the list as it is right now:

- Start menu, pause screen, game over screen with retry.
- Sound effects.
- Difficulty levels (speed increase).
- Different food types/bonuses.
- Obstacles on the board.
- High score tracking.
- Smoother movement/animation.

### Graphics/Game Engine

I'm going to use `piston_window`, here's a quote from its docs

> [With Piston, you should be able to get something up on the screen quickly, make a prototype, load images and sounds, polish it to look like a real game and port it to other platforms.](https://github.com/PistonDevelopers/piston/blob/master/GUIDE.md#:~:text=With%20Piston%2C%20you%20should%20be%20able%20to%20get%20something%20up%20on%20the%20screen%20quickly%2C%20make%20a%20prototype%2C%20load%20images%20and%20sounds%2C%20polish%20it%20to%20look%20like%20a%20real%20game%20and%20port%20it%20to%20other%20platforms.)

### Other Libraries that I'll Probably Need

Let's talk implementation! for the randomly spawning food I'll need
to generate random numbers, I know just the crate for that!

Also I need to think of someway to store the snake's position, this will
need to be updated every frame when the snake moves.. let me look into it!?
Alright! after some research, I reckon a `std::collections::VecDeque`
will get the job done.

### High-Level Blueprints

A well-structured, well-planned project is easier to build, modularity is
a good design principle to utilize here, ecce magnum opus:

#### **`main.rs`** (The Orchestrator)

This mod should only contain the high-level logic of putting components
of the game together, here's exactly how:

- Handles window setup using `piston_window`.
- Manages the main game loop: handling events and inputs, updates, renders.
- Creates and manages the main Game instance.

#### **`game.rs`** (The Game Manager)

- Holds the overall game state (snake position and heading, food position,
  score, game status).
- Contains the core game logic:
  - Updating game state.
  - Checking collisions (wall, self, food).
  - Spawning food.
  - Handling input events (delegating those to the snake).
  - Drawing all game elements.

#### **`snake.rs`** (The Snake's Brain)

- Defines the Snake struct (its position, segments, direction, growth state).
- Contains methods specific to the snake: moving, changing direction, growing,
  checking for self-collision, drawing itself.
- Defines the Direction enum.

#### **`food.rs`** (The Delicious Treat)

- Defines the Food struct (its position).
- Contains methods specific to food: creating new food, drawing itself.

### The Game Loop

- Read Input: Poll for user input (keyboard presses for direction changes).
- Update: Update the game's state (move snake, check collisions, consume
  food, update score).
- Render: Draw all game elements to the screen.

## Checklist of Tasks

My strategy is to build this piece by piece, and here's exactly how:

- [X] Start Simple: Get a basic window to open.
- [ ] Add Static Elements: Draw the snake and food at fixed positions.
- [ ] Introduce Movement: Make the snake move automatically in one direction.
- [ ] Add Control: Implement player input to change the snake's direction.
- [ ] Implement Core Mechanics: Add food eating, growth, and scoring.
- [ ] Add Game Over Conditions: Wall and self-collision detection.
- [ ] Refine and Polish: Adjust speeds, add visual cues.
