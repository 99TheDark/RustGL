mod model;
mod transformation;

use glium::Surface;
use std::fs;
use transformation::Matrix4;

#[macro_use]
extern crate glium;

pub fn read_shader(path: &str) -> String {
    let shader_path = format!("{}{}", "shader/", path);
    fs::read_to_string(shader_path).expect("Unexpected error reading shader file")
}

// Going to start copying + pasting the tutorial, and then go through and understand and modify the code given
fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(1600, 1000)
        .with_title("RustGL")
        .build(&event_loop);

    // Load image
    let path = &include_bytes!("../object/textures/crate.png");
    let image = image::load(std::io::Cursor::new(path), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let image_dims = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dims);
    let texture_array = glium::texture::Texture2d::new(&display, image).unwrap();
    let texture = texture_array
        .sampled()
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest); // Use nearest neighbor for crispy pixels

    // Load model
    let (vertices, normals, indices) = model::load_model("cube.obj", &display);

    // Setup OpenGL
    let vertex_shader_source = &read_shader("shader.vert");
    let fragment_shader_source = &read_shader("shader.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_source, fragment_shader_source, None)
            .unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        ..Default::default()
    };

    let mut time = 0.0;
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                // 60fps
                time += 1.0 / 60.0;

                // Aspect ratio
                let size = window.inner_size();
                let aspect = size.width as f32 / size.height as f32;

                let mut model = Matrix4::identity();
                model.scale(0.4, 0.4, 0.4);
                model.translate(0.0, -0.5, 3.5);
                model.rotate_x(-0.4);
                model.rotate_y(time);
                model.rotate_z(0.2);

                let projection = Matrix4::perspective(size.width, size.height, 0.1, 1024.0);

                let view =
                    transformation::view(&[0.0, f32::sin(time) * 0.3, 0.0], &[0.0, 0.0, 1.0]);

                let uniforms = uniform! {
                    modelMatrix: model.to_array(),
                    projectionMatrix: projection.to_array(),
                    viewMatrix: view,
                    surfaceTexture: texture,
                    aspect: aspect,
                };

                // Render
                let mut screen = display.draw();
                screen.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                screen
                    .draw(
                        (&vertices, &normals),
                        &indices,
                        &program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
                screen.finish().unwrap();

                // Loop
                window.request_redraw();
            }
            _ => (),
        };
    });
}
