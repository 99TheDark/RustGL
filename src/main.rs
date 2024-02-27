mod transformation;
mod vertex;

use glium::Surface;
use std::fs;
use transformation::translate;
use vertex::Vertex;

#[macro_use]
extern crate glium;

pub fn read_shader(path: &str) -> String {
    let shader_path = format!("{}{}", "shader/", path);
    let shader_content =
        fs::read_to_string(shader_path).expect("Unexpected error reading shader file");
    return shader_content;
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

    // Create shape
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        color: [0.0, 1.0, 0.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        color: [0.0, 0.0, 1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    // Setup OpenGL
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = &read_shader("shader.vert");
    let fragment_shader_src = &read_shader("shader.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

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

                let uniforms = uniform! {
                    translationMatrix: translate(0.3 * f32::sin(time), 0.0),
                };

                // Render
                let mut screen = display.draw();
                screen.clear_color(0.0, 0.0, 0.0, 1.0);
                screen
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &Default::default(),
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
