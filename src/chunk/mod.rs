use crate::vertex::Vertex;

pub struct Chunk {
    data: [[[u8; 64]; 64]; 64],
    X: i8,
    Y: i8,
    loaded: bool,
    pub vertex_data: Vec<Vertex>,
    pub index_data: Vec<u16>,
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut tmp = Chunk {
            data: [[[1; 64]; 64]; 64],
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
        tmp.update_all();
        return tmp;
    }

    pub fn update_all(&mut self) {
        self.vertex_data = Vec::new();
        self.index_data = Vec::new();
        let mut i = 0;
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    if self.data[x][y][z] == 1 {
                        self.vertex_data.push(Vertex::new([(x as i8)*self.X, y as i8, (z as i8)*self.Y], [0,0]));
                        self.index_data.push(i);
                        i += 1;
                    }
                }
            }
        }
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
