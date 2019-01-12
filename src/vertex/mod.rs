impl Vertex {
    pub fn new(pos: [f32; 3], tc: [i8; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_tex_coord: tc,
        }
    }
}

gfx_vertex_struct!( Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_tex_coord: [i8; 2] = "a_tex_coord",
});
