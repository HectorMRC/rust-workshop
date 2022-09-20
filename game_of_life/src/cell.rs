use std::marker::PhantomData;

const STAY_ALIVE_MIN_NEIGHBOURS: usize = 2;
const STAY_ALIVE_MAX_NEIGHBOURS: usize = 3;
const REVIVE_MIN_NEIGHBOURS: usize = 3;
const REVIVE_MAX_NEIGHBOURS: usize = 3;

#[derive(Debug)]
pub(crate) struct Alive;

#[derive(Debug)]
pub(crate) struct Dead;

#[derive(Debug)]
pub(crate) struct Cell<State = Dead> {
    state: PhantomData<State>,
    neighbours: Vec<usize>,
}

impl<State> Cell<State> {
    fn count_alive_neighbours(&self) -> usize {
        todo!()
    }
}

impl<'a> Cell<Alive> {
    pub fn next(self) -> Option<Cell<Dead>> {
        let count_alive = self.count_alive_neighbours();
        if count_alive >= STAY_ALIVE_MIN_NEIGHBOURS && count_alive <= STAY_ALIVE_MAX_NEIGHBOURS {
            return None;
        }

        Some(Cell::<Dead> {
            state: PhantomData,
            neighbours: self.neighbours,
        })
    }
}

impl<'a> Cell<Dead> {
    pub fn new(neighbours: Vec<usize>) -> Self {
        Self {
            state: PhantomData,
            neighbours: neighbours,
        }
    }

    pub fn next(self) -> Option<Cell<Alive>> {
        let count_alive = self.count_alive_neighbours();
        if count_alive > REVIVE_MAX_NEIGHBOURS || count_alive < REVIVE_MIN_NEIGHBOURS {
            return None;
        }

        Some(Cell::<Alive> {
            state: PhantomData,
            neighbours: self.neighbours,
        })
    }
}
