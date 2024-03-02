mod camera;
mod keyboard;
mod model;
mod transformation;
mod utils;

use camera::Camera;
use glium::Surface;
use keyboard::Keyboard;
use std::fs;
use transformation::Matrix4;
use winit::keyboard::KeyCode;

use crate::utils::b2f;

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
    let mut camera = Camera::new();
    let mut keyboard = Keyboard::new();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                } => {
                    if state == winit::event::ElementState::Pressed
                        && button == winit::event::MouseButton::Left
                    {
                        window.set_cursor_visible(false);
                        window
                            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                            .unwrap();
                    }
                }
                winit::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    ..
                } => match event.state {
                    winit::event::ElementState::Pressed => match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                            window.set_cursor_visible(true);
                            window
                                .set_cursor_grab(winit::window::CursorGrabMode::None)
                                .unwrap();
                        }
                        _ => keyboard.press(event.physical_key),
                    },
                    winit::event::ElementState::Released => {
                        keyboard.release(event.physical_key);
                    }
                },
                _ => (),
            },
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    camera.rotate(delta);
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                // 60fps
                time += 1.0 / 60.0;

                // Aspect ratio
                let size = window.inner_size();

                if keyboard.any_key_down() {
                    let x = b2f(keyboard.down(KeyCode::KeyD)) - b2f(keyboard.down(KeyCode::KeyA));
                    let y = b2f(keyboard.down(KeyCode::KeyE)) - b2f(keyboard.down(KeyCode::KeyQ));
                    let z = b2f(keyboard.down(KeyCode::KeyW)) - b2f(keyboard.down(KeyCode::KeyS));

                    camera.step(x, y, z);
                }

                let mut model = Matrix4::identity();
                model.scale(0.4, 0.4, 0.4);
                model.translate(0.0, -0.5 + f32::sin(time) * 0.3, 3.5);
                model.rotate_x(-0.4);
                model.rotate_y(time);
                model.rotate_z(0.2);

                let projection = Matrix4::perspective(size.width, size.height, 0.1, 1024.0);

                let view = transformation::view(&camera.position, &camera.direction);

                let uniforms = uniform! {
                    modelMatrix: model.to_array(),
                    projectionMatrix: projection.to_array(),
                    viewMatrix: view,
                    surfaceTexture: texture,
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
