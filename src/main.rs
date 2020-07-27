mod maze;
mod maze_maker;
mod maze_state;
mod terminal;

use maze_state::MazeState;
use std::time::SystemTime;

fn main() {
    let mut seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
    let maze = maze_maker::make_maze(8, 5, seed);
    let mut maze_state = MazeState::new(maze);

    terminal::mount_terminal(&mut move |k| {
        if let Some(k) = k {
            let k = match k {
                terminal::KeyEvent::Left => maze_state::KeyEvent::Left,
                terminal::KeyEvent::Right => maze_state::KeyEvent::Right,
                terminal::KeyEvent::Up => maze_state::KeyEvent::Up,
                terminal::KeyEvent::Down => maze_state::KeyEvent::Down,
            };
            maze_state.mv(k);
            if maze_state.is_goal() {
                seed += 1;
                let maze = maze_maker::make_maze(8, 5, seed);
                maze_state = MazeState::new(maze);
            }
        }
        format!("{}- Arrow key to move.\r\n- q key to exit.", maze_state)
    });
}
