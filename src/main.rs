#![feature(box_syntax)]

extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate shader_version;
extern crate rand;
extern crate fps_counter;

mod vertex;
use crate::vertex::Vertex;
use rand::Rng;
pub mod chunk;
pub mod world;
use fps_counter::FPSCounter;
use std::time::{Duration, Instant};
use std::num;

//-----------------------------------------
// setting up vertex data and the pipeline;

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    u_view_dir: gfx::Global<[f32; 3]> = "u_view_dir",
    u_tex_size: gfx::Global<f32> = "u_tex_size",
    u_atlas_size: gfx::Global<f32> = "u_atlas_size",
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
        WindowSettings::new("Retical", [1280, 720])
        .exit_on_esc(true)
        //.samples(4) //TODO: read documentation lol, gotta check if this is AA
        .opengl(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    let ref mut factory = window.factory.clone();

    //let c = chunk::Chunk::new();
    let mut w = world::World::new();

    //let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&(c.vertex_data), c.index_data.as_slice());
    //let vbuf = factory.create_vertex_buffer(&(c.vertex_data));
    //let slice = gfx::Slice::new_match_vertex_buffer(&vbuf);

    /*
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
    */

    let texture = Texture::from_path(factory, "../assets/textures/pixel_perfection.png", Flip::None, &TextureSettings::new()).unwrap();
    let texture_view = texture.view;

    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Scale,
        gfx::texture::WrapMode::Clamp);

    let glsl = opengl.to_glsl();

    let shader_set = factory.create_shader_set_geometry(
        Shaders::new().set(GLSL::V1_50, include_str!("../assets/shaders/vertex.glsl")).get(glsl).unwrap().as_bytes(),
        Shaders::new().set(GLSL::V1_50, include_str!("../assets/shaders/geometry.glsl")).get(glsl).unwrap().as_bytes(),
        Shaders::new().set(GLSL::V1_50, include_str!("../assets/shaders/fragment.glsl")).get(glsl).unwrap().as_bytes()
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
        [-1.0, 1.0, -1.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let mut fps_c = FPSCounter::new();
    let mut fps: usize = 0;
    let mut total_time = 0;
    let mut delta_time = 0;

    let mut glyphs = Glyphs::new("../assets/fonts/OpenSans/OpenSans-Light.ttf", window.factory.clone(), TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        let now = Instant::now();

        first_person.event(&e);

        window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
        window.encoder.clear_depth(&window.output_stencil, 1.0);

        for i in 0..w.loaded_chunks.len() {
            let height = (total_time as f64).sin() * 5.0;
            w.chunks[w.loaded_chunks[i]].set_block_u32(0, 0, 0, if height > 0.0 {0} else {1});
            let vertex_data = w.chunks[w.loaded_chunks[i]].vertex_data.clone();
            let index_data = w.chunks[w.loaded_chunks[i]].index_data.clone();
            let (vbuf, slice) = window.factory.create_vertex_buffer_with_slice(&(vertex_data), index_data.as_slice());

            let mut data = pipe::Data {
                    vbuf: vbuf.clone(),
                    u_model_view_proj: [[0.0; 4]; 4],
                    u_view_dir: [0.0; 3],
                    u_tex_size: 16.0,
                    u_atlas_size: 256.0,
                    t_color: (texture_view.clone(), factory.create_sampler(sinfo)),
                    out_color: window.output_color.clone(),
                    out_depth: window.output_stencil.clone(),
                };

            window.draw_3d(&e, |window| {
                let args = e.render_args().unwrap();

                data.u_model_view_proj = model_view_projection(
                    model,
                    first_person.camera(args.ext_dt).orthogonal(),
                    projection
                );
                data.u_view_dir = first_person.direction;

                //window.encoder.draw(&slice, &pso, &data);
                //w.draw(first_person.position[0], first_person.position[2], window);

                window.encoder.draw(&slice, &pso, &data);
            });

            if let Some(_) = e.resize_args() {
                projection = get_projection(&window);
                data.out_color = window.output_color.clone();
                data.out_depth = window.output_stencil.clone();
            }
        }

        window.draw_2d(&e, |c, g| {
            let transform_fps = c.transform.trans(5.0, 16.0);
            let transform_ms = c.transform.trans(5.0, 32.0);

            //clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16).draw(
                &*format!("{} FPS", fps),
                &mut glyphs,
                &c.draw_state,
                transform_fps, g
            ).unwrap();
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16).draw(
                &*format!("{} MS", delta_time),
                &mut glyphs,
                &c.draw_state,
                transform_ms, g
            )
        });

        delta_time = now.elapsed().as_millis();
        total_time += delta_time;
        fps = fps_c.tick();
    }
}
