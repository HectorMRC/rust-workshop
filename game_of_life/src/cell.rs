use std::marker::PhantomData;

const MIN_NEIGHBOURS: usize = 2;
const MAX_NEIGHBOURS: usize = 3;
const NEIGHBOURS_NEEDED_TO_REVIVE: usize = 3;

#[derive(Clone)]
pub(crate) struct Alive;

#[derive(Clone)]
pub(crate) struct Dead;

#[derive(Clone)]
pub(crate) struct Cell<State = Dead> {
    state: PhantomData<State>,
    pub(crate) neighbours: Vec<usize>,
}

impl Default for Cell<Dead> {
    fn default() -> Self {
        Cell {
            state: PhantomData,
            neighbours: Vec::new(),
        }
    }
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

    pub(crate) fn next(&self, neighborhood: &[Cell]) -> Option<Cell<Dead>> {
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
    pub(crate) fn next(&self, neighborhood: &[Cell]) -> Option<Cell<Alive>> {
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
}
