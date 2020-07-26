use crate::maze::Maze;

#[derive(Debug)]
pub enum KeyEvent {
    Left,
    Right,
    Up,
    Down,
}

pub struct MazeState {
    maze: Maze,
    position: (usize, usize),
}

impl MazeState{
    pub fn new(maze: Maze) -> MazeState {
        let position = maze.start.clone();
        MazeState{
            maze,
            position,
        }
    }

    pub fn mv(&mut self, key: KeyEvent) {
        let (x, y) = self.position;
        let p = match key {
            KeyEvent::Left => (x - 1, y),
            KeyEvent::Right => (x + 1, y),
            KeyEvent::Up => (x, y - 1),
            KeyEvent::Down => (x, y + 1),
        };
        if !self.maze.field[p.1 * self.maze.width + p.0] {
            self.position = p;
        }
    }

    pub fn is_goal(&self) -> bool {
        self.position == self.maze.goal
    }
}

use std::fmt;

impl fmt::Display for MazeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maze = &self.maze;
        for i in 0..maze.height {
            for j in 0..maze.width {
                let p = (j, i);
                let char = if self.position == p {
                    '@'
                } else if maze.start == p {
                    'S'
                } else if maze.goal == p {
                    'G'
                } else if maze.field[i * maze.width + j] {
                    '#'
                } else {
                    ' '
                };
                write!(f, "{}", char)?;
            }
            write!(f, "\r\n")?;
        }
        write!(f, "")
    }
}
