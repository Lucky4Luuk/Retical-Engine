use crate::chunk::Chunk;

pub struct World {
    pub chunks: Vec<Chunk>,
    pub loaded_chunks: Vec<usize>,
}

impl World {
    pub fn new() -> World {
        let mut tmp = World {
            chunks: Vec::new(),
            loaded_chunks: Vec::new(),
        };
        tmp.chunks.push(Chunk::new(0,0));
        tmp.chunks.push(Chunk::new(1,0));
        tmp.update();
        return tmp;
    }

    pub fn update(&mut self) {
        self.loaded_chunks = Vec::new();
        for i in 0..self.chunks.len() {
            //TODO: Should check the distance to the chunks
            //to determine if they should be loaded
            if self.chunks[i].loaded {
                self.loaded_chunks.push(i);
            }
        }
    }
}
