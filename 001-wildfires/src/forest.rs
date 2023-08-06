use fastrand;
#[derive(Clone,Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone,Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone,Copy,PartialEq)]
pub enum State {
    Empty,
    Tree,
    Burning(u8),
}

#[derive(Clone,Copy)]
pub struct Plot {
    pub position: Position,
    pub state: State,
    prob_ignite: u8,
}

#[derive(Clone)]
pub struct Forest {
    pub land: Vec<Vec<Plot>>
}

impl Forest {
    pub fn new(dimensions: Dimensions, fire_start: (usize,usize), prob_ignite: u8) -> Forest {
        let mut land: Vec<Vec<Plot>> = Vec::with_capacity(dimensions.width);
        for x in 0..dimensions.height {
            let mut cols: Vec<Plot> = Vec::with_capacity(dimensions.height);
            for y in 0..dimensions.height {
                cols.push(Plot {
                    position: Position {
                        x: x as f32,
                        y: y as f32,
                    },
                    state: match (x,y) {
                        //Boundary conditions:
                        (0,_) => State::Empty,
                        (_,0) => State::Empty,
                        _ if x == dimensions.width-1 => State::Empty,
                        _ if y == dimensions.height-1 => State::Empty,
                        //Intial conditions:
                        _ if (x,y) == fire_start => State::Burning(5),
                        (_,_) => State::Tree,
                    },
                    prob_ignite,
                });
            }
            land.push(cols);
        }
        Forest { land }
    }

    pub fn update(&mut self) {
        for x in 0..self.land.len() {
            for y in 0..self.land[x].len() {

                self.land[x][y].state = match self.land[x][y].state {
                    State::Empty => State::Empty,
                    State::Burning(0) => State::Empty,
                    State::Burning(iter) => State::Burning(iter-1),
                    State::Tree => {
                        let neighbour_burning = [
                            self.land[x][y-1].state,
                            self.land[x-1][y].state,
                            self.land[x][y+1].state,
                            self.land[x+1][y].state,
                        ].contains(&State::Burning(4));
                        let ignite = fastrand::u8(0..255) < self.land[x][y].prob_ignite;
                        if neighbour_burning && ignite {
                            State::Burning(5)
                        } else {
                            State::Tree
                        }
                    }
                };
            }
        }
    }
}
