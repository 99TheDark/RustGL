#[macro_use]
extern crate glium;

use glium::Surface;
use std::fs;
use vertex::Vertex;

pub mod vertex {
    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
}

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
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(1600, 1000)
        .with_title("RustGL")
        .build(&event_loop);

    // Create shape
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    // Setup OpenGL
    let mut frame = display.draw();
    frame.clear_color(1.0, 1.0, 1.0, 1.0);
    frame.finish().unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = &read_shader("shader.vert");
    let fragment_shader_src = &read_shader("shader.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
