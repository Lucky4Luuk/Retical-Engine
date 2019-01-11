use crate::vertex::Vertex;
use rand::Rng;
use std::num;

pub struct Chunk {
    data: [[[u8; 64]; 64]; 64],
    X: i16,
    Y: i16,
    loaded: bool,
    pub vertex_data: Vec<Vertex>,
    pub index_data: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut tmp = Chunk {
            data: [[[0; 64]; 64]; 64],
            X: 0,
            Y: 0,
            loaded: true,
            vertex_data: vec![
            Vertex::new([-1,-1,-1], [0,0]),
            Vertex::new([ 0, 0,-1], [0,0]),
            Vertex::new([ 1,-1,-1], [0,0]),
            ],
            index_data: vec![0, 1, 2],
        };
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    tmp.data[x][y][z] = rand::thread_rng().gen_range(1, 4) as u8;
                }
            }
        }
        tmp.update_all();
        return tmp;
    }

    pub fn update_block(&mut self, x: u8, y: u8, z: u8, block_type: u8) {
        if x < 64 && y < 64 && z < 64 {
            self.data[x as usize][y as usize][z as usize] = block_type;
        }

        //Update the chunk fully because I haven't yet
        //figured out how I want to remove a certain vertex from the vertex list
        self.update_all();
    }

    pub fn update_all(&mut self) {
        self.vertex_data = Vec::new();
        self.index_data = Vec::new();
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    let mut next_to_air = false;
                    //Check if at the border of a chunk
                    if x == 0 || y == 0 || z == 0 || x == 63 || y == 63 || z == 63 {
                        next_to_air = true;
                    }
                    //Check air blocks
                    if next_to_air != true {
                        if self.data[x-1][y][z] == 0 || self.data[x+1][y][z] == 0 || self.data[x][y-1][z] == 0 || self.data[x][y+1][z] == 0 || self.data[x][y][z-1] == 0 || self.data[x][y][z+1] == 0 {
                            next_to_air = true;
                        }
                    }

                    if next_to_air {
                        if self.data[x][y][z] == 1 {
                            self.vertex_data.push(Vertex::new([(x as i16)+(self.X*64), y as i16, (z as i16)+(self.Y*64)], [0,0]));
                        } else if self.data[x][y][z] == 2 {
                            self.vertex_data.push(Vertex::new([(x as i16)+(self.X*64), y as i16, (z as i16)+(self.Y*64)], [1,0]));
                        } else if self.data[x][y][z] == 3 {
                            self.vertex_data.push(Vertex::new([(x as i16)+(self.X*64), y as i16, (z as i16)+(self.Y*64)], [2,0]));
                        }
                    }
                }
            }
        }
        for i in 0..self.vertex_data.len() {
            self.index_data.push(i as u32);
        }
        println!("{} vertices", self.vertex_data.len());
        println!("{} indices", self.index_data.len());
        //TODO: Make this actually gather all vertices in here, aka all voxels. Will be abstracted into chunk classes however.
        //RhoneRanger had a great suggestion.
        //If we order the data according to the block type, we could use a simple
        //if-elseif statement in the geometry shader to define multiple block shapes.
        //Example for Minecraft blocks: block type 0 is the normal block, then block type 1 is grass/flowers, block type 2 is the fences
        //Another option is to encode the blocks as a 16x16x16 3D texture, so that each block has it's own fully fledged 3D voxel model.
        //That'd make more sense for an actual engine, perhaps.
        //I could combine this, for a performance increase.
    }
}
