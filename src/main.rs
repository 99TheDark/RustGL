use glium::Surface;
use winit::dpi::LogicalSize;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // Better way to do this?
    window.set_min_inner_size(Some(LogicalSize::new(800.0, 500.0)));

    let mut frame = display.draw();
    frame.clear_color(0.3, 0.4, 0.8, 1.0);
    frame.finish().unwrap();

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
