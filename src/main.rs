mod terminal;
mod maze;
mod maze_maker;
mod maze_state;

use maze_state::MazeState;

fn main() {
    let maze = maze_maker::make_maze(17, 11);
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
        }
        format!("{}- Arrow key to move.\r\n- q key to exit.", maze_state)
    });
}
