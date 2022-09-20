use super::cell;

pub(crate) struct Grid {
    cells: Vec<cell::Cell>,
}

impl Grid {
    pub fn new(rows: i32, columns: i32) -> Self {
        let size = (rows * columns).abs();
        let mut grid = Self {
            cells: Vec::with_capacity(size as usize),
        };

        (0..size).for_each(|index| {
            grid.cells.push(cell::Cell::new(vec![
                (index - columns).rem_euclid(size) as usize,     // top
                (index + columns).rem_euclid(size) as usize,     // bottom
                (index - 1).rem_euclid(size) as usize,           // left
                (index + 1).rem_euclid(size) as usize,           // right
                (index - columns - 1).rem_euclid(size) as usize, // top-left
                (index - columns).rem_euclid(size) as usize,     // top-right
                (index + columns).rem_euclid(size) as usize,     // bottom-left
                (index + columns + 1).rem_euclid(size) as usize, // bottom-right
            ]));
        });

        grid
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn new_should_not_fail() {
        vec![(-3, 5, 15_usize), (3, 5, 15_usize), (50, 100, 5000_usize)]
            .into_iter()
            .for_each(|(rows, columns, size)| {
                let got = super::Grid::new(rows, columns);
                assert_eq!(got.cells.len(), size);
            });
    }
}
