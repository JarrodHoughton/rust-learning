use macroquad::prelude::*;
use macroquad::rand::gen_range;

const GRID_SIZE: usize = 50;
const CELL_SIZE: usize = 10;
const WINDOW_WIDTH: f32 = (GRID_SIZE * CELL_SIZE) as f32;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut is_press_to_play = false;
    let mut grid = (0..(GRID_SIZE * GRID_SIZE))
        .map(|_| gen_range(0, 10) < 4)
        .collect();
    
    loop {
        clear_background(BLACK);
        
        draw_grid(&mut grid);

        // render frames at screen refresh rate
        if !is_press_to_play {
            process_ruleset(&mut grid);
        // render frames when enter key is pressed down
        } else if is_key_down(KeyCode::Enter) {
            process_ruleset(&mut grid);
        }

        // show helper text
        draw_fps(20.0);
        draw_controls(40.0);
        draw_current_mode(is_press_to_play, 120.0);

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let x = mouse_pos.0 as usize / CELL_SIZE;
            let y = mouse_pos.1  as usize / CELL_SIZE;

            if x < GRID_SIZE && y < GRID_SIZE {
                grid[pos_to_index(x, y)] = true;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let x = mouse_pos.0 as usize / CELL_SIZE;
            let y = mouse_pos.1  as usize / CELL_SIZE;

            if x < GRID_SIZE && y < GRID_SIZE {
                grid[pos_to_index(x, y)] = false;
            }
        }

        // toggle play mode
        if is_key_pressed(KeyCode::Space) {
            is_press_to_play = !is_press_to_play;
        }

        next_frame().await
    }
}

fn draw_controls (y_offset: f32) {
    let y_spacing = 20.0;
    draw_text("Controls:", WINDOW_WIDTH, y_offset, 20.0, WHITE);
    draw_text("Press SPACE to toggle play mode: Live simulation or Press to play", WINDOW_WIDTH, y_offset + y_spacing, 20.0, WHITE);
    draw_text("Left click to add cells, right click to remove cells", WINDOW_WIDTH, y_offset + y_spacing*2.0, 20.0, WHITE);
    draw_text("Hold ENTER to step through generations in 'Press to play' mode", WINDOW_WIDTH, y_offset + y_spacing*3.0, 20.0, WHITE);
}

fn draw_current_mode(is_press_to_play: bool, y_offset: f32) {
    if is_press_to_play {
        draw_text("Mode: Press to play mode", WINDOW_WIDTH, y_offset, 20.0, WHITE);
    } else {
        draw_text("Mode: Live simulation", WINDOW_WIDTH, y_offset, 20.0, WHITE);
    }
}

fn draw_fps(y_offset: f32) {
    let fps = get_fps();
    draw_text(&format!("FPS: {}", fps), WINDOW_WIDTH, y_offset, 20.0, WHITE);
}

fn process_ruleset(grid: &mut Vec<bool>) {
    let mut new_grid = grid.clone();
    for i in 0..grid.len() {
        let (x, y) = index_to_pos(i);
        let alive_neighbors = count_alive_neighbors(grid, x, y);
        new_grid[i] = match (grid[i], alive_neighbors) {
            // Any live cell with two or three live neighbours lives on to the next generation.
            (true, 2..=3) => true,
            // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
            (false, 3) => true,
            // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
            // Any live cell with more than three live neighbours dies, as if by overpopulation.
            _ => false,
        };
    }
    *grid = new_grid;
}

fn count_alive_neighbors(grid: &Vec<bool>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dx in -1..=1 {

        // check bounds
        if {(dx + x as isize) < 0} || {(dx + x as isize) >= GRID_SIZE as isize} {
            continue;
        }
        for dy in -1..=1 {

            // check bounds
            if {(dy + y as isize) < 0} || {(dy + y as isize) >= GRID_SIZE as isize} {
                continue;
            }

            // ignore the cell itself
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if grid[pos_to_index(nx, ny)] {
                count += 1;
            }
        }
    }
    count
}

fn index_to_pos(index: usize) -> (usize, usize) {
    let x = index % GRID_SIZE;
    let y = index / GRID_SIZE;
    (x as usize, y as usize)
}

fn pos_to_index(x: usize, y: usize) -> usize {
    y * GRID_SIZE + x
}

fn draw_grid(grid: &mut Vec<bool>) {
    for (i, is_alive) in grid.iter().enumerate() {
        if *is_alive {
            let (x, y) = index_to_pos(i);
            draw_cell(x, y);
        }
    }
}

fn draw_cell(x: usize, y: usize) {
    draw_rectangle(
        (x * CELL_SIZE) as f32,
        (y * CELL_SIZE) as f32,
        CELL_SIZE as f32,
        CELL_SIZE as f32,
        WHITE,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_to_pos() {
        assert_eq!(index_to_pos(0), (0, 0));
        assert_eq!(index_to_pos(1), (1, 0));
        assert_eq!(index_to_pos(10), (0, 1));
        assert_eq!(index_to_pos(11), (1, 1));
    }

    #[test]
    fn test_pos_to_index() {
        assert_eq!(pos_to_index(0, 0), 0);
        assert_eq!(pos_to_index(1, 0), 1);
        assert_eq!(pos_to_index(0, 1), 10);
        assert_eq!(pos_to_index(1, 1), 11);
    }
}
