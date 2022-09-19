use std::marker::PhantomData;

const MIN_NEIGHBOURS: usize = 2;
const MAX_NEIGHBOURS: usize = 3;
const NEIGHBOURS_NEEDED_TO_REVIVE: usize = 3;

struct Alive;

struct Dead;

#[derive(Default)]
struct Cell<State = Dead> {
    state: PhantomData<State>,
    neighbours: Vec<usize>,
}

impl<State> Cell<State> {
    fn proceed<NextState>(&self) -> Cell<NextState> {
        Cell {
            state: PhantomData,
            neighbours: self.neighbours.clone(),
        }
    }

    fn stay(&self) -> Self {
        Self {
            state: PhantomData,
            neighbours: self.neighbours.clone(),
        }
    }
}

impl Cell<Alive> {
    fn is_alive(&self) -> bool {
        true
    }

    fn die(&self, neighborhood: &[Cell]) -> Option<Cell<Dead>> {
        let alive_neighbours = self
            .neighbours
            .iter()
            .filter(|neighbour| neighborhood[**neighbour].is_alive())
            .count();

        if MAX_NEIGHBOURS >= alive_neighbours && alive_neighbours >= MIN_NEIGHBOURS {
            return None;
        }

        Some(self.proceed::<Dead>())
    }
}

impl Cell<Dead> {
    fn revive(&self, neighborhood: &[Cell]) -> Option<Cell<Alive>> {
        let alive_neighbours = self
            .neighbours
            .iter()
            .filter(|neighbour| neighborhood[**neighbour].is_alive())
            .count();

        if NEIGHBOURS_NEEDED_TO_REVIVE == alive_neighbours {
            return Some(self.proceed::<Alive>());
        }

        None
    }

    fn is_alive(&self) -> bool {
        false
    }

    fn next(&self) -> Cell {
        todo!()
    }
}
