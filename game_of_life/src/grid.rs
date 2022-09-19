use crate::cell::Cell;

struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn new(n: usize, m: usize) -> Self {
        let cells: Vec<Cell> = vec![Cell::default(); n * m];
        cells.iter().map(|c| c);
        // for x in 0..n * m {}
        Grid { cells }
    }

    fn iterate(&self, max: usize) {
        for i in 0..max {
            let new_cells: Vec<Cell> = self
                .cells
                .iter()
                .map(|cell| cell.next(&self.cells).unwrap_or(cell.clone()))
                .collect::<Vec<Cell>>();
        }
    }
}
