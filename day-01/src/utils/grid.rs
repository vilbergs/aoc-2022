pub struct Grid<T> {
    width: u32,
    height: u32,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new<F>(size: u32) -> Self {
        let cells = Vec::new();

        Self {
            width: size,
            height: size,
            cells,
        }
    }
}
