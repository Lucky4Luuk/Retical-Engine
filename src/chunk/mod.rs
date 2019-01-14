use crate::vertex::Vertex;
use rand::Rng;
use std::num;

pub struct Chunk {
    data: std::boxed::Box<[[[ [usize; 3]; 64]; 64]; 64]>,
    X: i32,
    Y: i32,
    pub loaded: bool,
    pub vertex_data: Vec<Vertex>,
    pub index_data: Vec<u32>,
}

impl Chunk {
    pub fn new(x: i32, y: i32) -> Chunk {
        let mut tmp = Chunk {
            data: box [[[ [0; 3]; 64]; 64]; 64],
            X: x,
            Y: y,
            loaded: true,
            vertex_data: vec![
            Vertex::new([-1.0,-1.0,-1.0], [0,0]),
            Vertex::new([ 0.0, 0.0,-1.0], [0,0]),
            Vertex::new([ 1.0,-1.0,-1.0], [0,0]),
            ],
            index_data: vec![0, 1, 2],
        };
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    tmp.data[x][y][z] = [rand::thread_rng().gen_range(1, 4), 0,0];
                }
            }
        }
        tmp.update_all();
        return tmp;
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block_type: usize) {
        if x < 64 && y < 64 && z < 64 {
            self.data[x as usize][y as usize][z as usize][0] = block_type;
            self.update_block(x as usize, y as usize, z as usize);

            /*
            if x > 0 {
                self.update_block((x-1) as usize, y as usize, z as usize);
            }
            if x < 63 {
                self.update_block((x+1) as usize, y as usize, z as usize);
            }

            if y > 0 {
                self.update_block(x as usize, (y-1) as usize, z as usize);
            }
            if y < 63 {
                self.update_block(x as usize, (y+1) as usize, z as usize);
            }

            if z > 0 {
                self.update_block(x as usize, y as usize, (z-1) as usize);
            }
            if z < 63 {
                self.update_block(x as usize, y as usize, (z+1) as usize);
            }
            */
        }

        //Update the chunk fully because I haven't yet
        //figured out how I want to remove a certain vertex from the vertex list
        //self.update_all();
    }

    pub fn update_block(&mut self, x: usize, y: usize, z: usize) {
        let mut next_to_air = false;
        //Check if at the border of a chunk
        if x == 0 || y == 0 || z == 0 || x == 63 || y == 63 || z == 63 {
            next_to_air = true;
        }
        //Check air blocks
        if next_to_air != true {
            if self.data[x-1][y][z][0] == 0 || self.data[x+1][y][z][0] == 0 || self.data[x][y-1][z][0] == 0 || self.data[x][y+1][z][0] == 0 || self.data[x][y][z-1][0] == 0 || self.data[x][y][z+1][0] == 0 {
                next_to_air = true;
            }
        }

        if next_to_air {
            let mut i = self.data[x][y][z][1]; //Index in vertex buffer

            if i == 0 {
                //println!("block {} {} {} ({}) doesn't exist in Vertex Buffer", x,y,z, i);
                if self.data[x][y][z][0] == 1 {
                    //println!("block {} {} {} ({}) has been created", x,y,z, i);
                    self.vertex_data.push(Vertex::new([(x as f32)+(self.X as f32)*64.0, y as f32, (z as f32)+(self.Y as f32)*64.0], [0,0]));
                } else if self.data[x][y][z][0] == 2 {
                    self.vertex_data.push(Vertex::new([(x as f32)+(self.X as f32)*64.0, y as f32, (z as f32)+(self.Y as f32)*64.0], [1,0]));
                } else if self.data[x][y][z][0] == 3 {
                    self.vertex_data.push(Vertex::new([(x as f32)+(self.X as f32)*64.0, y as f32, (z as f32)+(self.Y as f32)*64.0], [2,0]));
                }

                if self.data[x][y][z][0] > 0 {
                    i = self.vertex_data.len() - 1;
                    self.index_data.push(i as u32);
                    self.data[x][y][z][1] = i;
                    self.data[x][y][z][2] = self.index_data.len() - 1;
                } else {
                    //println!("block {} {} {} ({}) is an air block", x,y,z, i);
                }
            } else {
                //println!("block {} {} {} ({}) already exists in Vertex Buffer", x,y,z, i);
                if self.data[x][y][z][0] == 0 {
                    //println!("block {} {} {} ({}) is an air block and will be deleted", x,y,z, i);
                    let j = self.data[x][y][z][2]; //Index in index buffer
                    self.vertex_data.remove(i-1);
                    self.index_data.remove(j-1);
                    self.data[x][y][z][1] = 0;
                    self.data[x][y][z][2] = 0;
                    //println!("{}", self.data[x][y][z][1]);
                } else if self.data[x][y][z][0] == 1 {
                    self.vertex_data[i-1] = Vertex::new([(x as f32)+(self.X as f32)*64.0, y as f32, (z as f32)+(self.Y as f32)*64.0], [0,0]);
                } else if self.data[x][y][z][0] == 2 {
                    self.vertex_data[i-1] = Vertex::new([(x as f32)+(self.X as f32)*64.0, y as f32, (z as f32)+(self.Y as f32)*64.0], [1,0]);
                } else if self.data[x][y][z][0] == 3 {
                    self.vertex_data[i-1] = Vertex::new([(x as f32)+(self.X as f32)*64.0, y as f32, (z as f32)+(self.Y as f32)*64.0], [2,0]);
                }
            }
        } else {
            let i = self.data[x][y][z][1]; //Index in vertex buffer
            let j = self.data[x][y][z][2]; //Index in index buffer
            if i > 0 {
                self.vertex_data.remove(i-1);
                self.index_data.remove(j-1);
                self.data[x][y][z][1] = 0;
                self.data[x][y][z][2] = 0;
            }
        }
    }

    pub fn get_block(self, x: u8, y: u8, z: u8) -> usize {
        return self.data[x as usize][y as usize][z as usize][0];
    }
    //get_block is kind of redundant as the data is already
    //accessible from the outside

    pub fn update_all(&mut self) {
        self.vertex_data = Vec::new();
        self.index_data = Vec::new();
        let mut i = 0;

        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    self.update_block(x,y,z);
                }
            }
        }
        //println!("{} vertices", self.vertex_data.len());
        //println!("{} indices", self.index_data.len());
        //RhoneRanger had a great suggestion.
        //If we order the data according to the block type, we could use a simple
        //if-elseif statement in the geometry shader to define multiple block shapes.
        //Example for Minecraft blocks: block type 0 is the normal block, then block type 1 is grass/flowers, block type 2 is the fences
        //Another option is to encode the blocks as a 16x16x16 3D texture, so that each block has it's own fully fledged 3D voxel model.
        //That'd make more sense for an actual engine, perhaps.
        //I could combine this, for a performance increase.
    }
}
