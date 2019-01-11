use crate::chunk::Chunk;
use rand::Rng;
use std::num;

pub struct World {
    pub chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> World {
        let mut tmp = World {
            chunks: Vec::new(),
        };
        tmp.chunks.push(Chunk::new());
        return tmp;
    }
}
