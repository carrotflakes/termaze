use crate::maze::Maze;

pub fn make_maze(width: usize, height: usize) -> Maze {
    let mut field = vec![false; width * height];

    // wall
    for y in 0..height {
        field[y * width] = true;
        field[y * width + width - 1] = true;
    }
    for x in 0..width {
        field[x] = true;
        field[(height - 1) * width + x] = true;
    }

    // crosses
    for y in 0..height {
        for x in 0..width {
            if x % 2 == 0 && y % 2 == 0 {
                field[y * width + x] = true;
            }
        }
    }


    Maze {
        width,
        height,
        field,
        start: (1, 1),
        goal: (width - 2, height - 2),
    }
}
