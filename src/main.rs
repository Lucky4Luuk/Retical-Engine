extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate shader_version;

mod vertex;
use crate::vertex::Vertex;
pub mod chunk;

//-----------------------------------------
// setting up vertex data and the pipeline;



gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

//-----------------------------------------

fn main() {
    use piston_window::*;
    use gfx::traits::*;
    use shader_version::Shaders;
    use shader_version::glsl::GLSL;
    use camera_controllers::{
        FirstPersonSettings,
        FirstPerson,
        CameraPerspective,
        model_view_projection
    };

    //set up opengl with the right version;
    let opengl = OpenGL::V4_1;

    let mut window: PistonWindow =
        WindowSettings::new("The Most Epic Minecraft Client Ever", [1280, 720])
        .exit_on_esc(true)
        //.samples(4) //TODO: read documentation lol, gotta check if this is AA
        .opengl(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    let ref mut factory = window.factory.clone();

    let c = chunk::Chunk::new();

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&(c.vertex_data), c.index_data.as_slice());

    let texels = [
        [0xff, 0xff, 0xff, 0x00],
        [0xff, 0x00, 0x00, 0x00],
        [0x00, 0xff, 0x00, 0x00],
        [0x00, 0x00, 0xff, 0x00]
    ];
    let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        gfx::texture::Kind::D2(2, 2, gfx::texture::AaMode::Single),
        gfx::texture::Mipmap::Provided,
        &[&texels]).unwrap();

    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Bilinear,
        gfx::texture::WrapMode::Clamp);

    let glsl = opengl.to_glsl();
    /*
    let pso = factory.create_pipeline_simple(
            Shaders::new()
                //.set(GLSL::V1_20, include_str!("../assets/cube_120.glslv"))
                .set(GLSL::V1_50, include_str!("../assets/vertex.glsl"))
                .get(glsl).unwrap().as_bytes(),
            Shaders::new()
                //.set(GLSL::V1_20, include_str!("../assets/cube_120.glslf"))
                .set(GLSL::V1_50, include_str!("../assets/fragment.glsl"))
                .get(glsl).unwrap().as_bytes(),
            pipe::new()
        ).unwrap();
    */
    let shader_set = factory.create_shader_set_geometry(
        Shaders::new().set(GLSL::V1_50, include_str!("../assets/vertex.glsl")).get(glsl).unwrap().as_bytes(),
        Shaders::new().set(GLSL::V1_50, include_str!("../assets/geometry.glsl")).get(glsl).unwrap().as_bytes(),
        Shaders::new().set(GLSL::V1_50, include_str!("../assets/fragment.glsl")).get(glsl).unwrap().as_bytes()
    ).unwrap();

    let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();

    let pso = factory.create_pipeline_state(&shader_set, gfx::Primitive::PointList, rasterizer, pipe::new()).unwrap();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };

    let model = vecmath::mat4_id();
    let mut projection = get_projection(&window);
    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let mut data = pipe::Data {
            vbuf: vbuf.clone(),
            u_model_view_proj: [[0.0; 4]; 4],
            t_color: (texture_view, factory.create_sampler(sinfo)),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };

    while let Some(e) = window.next() {
        first_person.event(&e);

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();

            window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(
                model,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            window.encoder.draw(&slice, &pso, &data);
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }
    }
}
