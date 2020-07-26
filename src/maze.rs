pub struct Maze{
    pub width: usize,
    pub height: usize,
    pub field: Vec<bool>,
    pub start: (usize, usize),
    pub goal: (usize, usize),
}
