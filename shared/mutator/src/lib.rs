// GRAB_ME: Basic Structure Definitions

// basic unit of OO in rust
// holds static members and traits

#[derive(Debug)]
pub struct Mutator {
    seed: u64,
    size: u64,
}

impl Mutator {
    pub fn new() -> Mutator {
        Mutator {
            seed: 100,
            size: 0
        }
    }

    pub fn seed(mut self, randint: u64) -> Self {
        self.seed = randint;
        self
    }

    pub fn max_input_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
}

struct Seed {

}

// traits are collections of methods
// hold method declarations and can declare default implementations 
// needs to be 'grafted' onto a struct with "impl for"
pub trait InputDatabase {
    fn num_inputs(&self) -> usize;
    fn input(&self, idx: usize) -> Option<&[u8]>;
}

pub struct EmptyDatabase {

}
