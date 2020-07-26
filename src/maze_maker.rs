use crate::maze::{Maze, BLOCK, EMPTY};
use rand_core::RngCore;
use rand_pcg::Pcg32;

pub fn make_maze(width: usize, height: usize, seed: u64) -> Maze {
    let (width, height) = (width * 2 + 1, height * 2 + 1);
    let mut field = vec![BLOCK; width * height];
    let start = (1, 1);
    let goal = (width - 2, height - 2);

    field[start.0 * width + start.1] = EMPTY;
    let mut ps = vec![start];
    let mut rnd = Pcg32::new(seed, 0xa02bdbf7bb3c0a7);

    while !ps.is_empty() {
        let p = ps.remove((rnd.next_u32() % ps.len() as u32) as usize);
        let mut can = Vec::with_capacity(4);
        if 1 < p.0 && field[p.1 * width + p.0 - 2] == BLOCK {
            can.push((p.0 - 2, p.1));
        }
        if p.0 < width - 2 && field[p.1 * width + p.0 + 2] == BLOCK {
            can.push((p.0 + 2, p.1));
        }
        if 1 < p.1 && field[(p.1 - 2) * width + p.0] == BLOCK {
            can.push((p.0, p.1 - 2));
        }
        if p.1 < height - 2 && field[(p.1 + 2) * width + p.0] == BLOCK {
            can.push((p.0, p.1 + 2));
        }
        while !can.is_empty() {
            let q = can.remove((rnd.next_u32() % can.len() as u32) as usize);
            field[(p.1 + q.1) / 2 * width + (p.0 + q.0) / 2] = EMPTY;
            field[q.1 * width + q.0] = EMPTY;
            ps.push(q);
            if rnd.next_u32() % 100 < 80 {
                break;
            }
        }
        if ps.is_empty() {
            'find_block:
            for y in 0..height {
                for x in 0..width {
                    if x % 2 == 1 && y % 2 == 1 {
                        if field[y * width + x] == BLOCK {
                            if 1 < x {
                                field[y * width + x - 1] = EMPTY;
                            } else if 1 < y {
                                field[(y - 1) * width + x] = EMPTY;
                            }
                            field[y * width + x] = EMPTY;
                            ps.push((x, y));
                            break 'find_block;
                        }
                    }
                }
            }
        }
    }

    Maze {
        width,
        height,
        field,
        start,
        goal,
    }
}
