use asteroids_screensaver::AsteroidsScreensaver;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::time::Instant;
use std::sync::Arc;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    // Get primary monitor size for automatic adaptation
    let primary_monitor = event_loop.available_monitors()
        .next()
        .expect("No monitors found");

    let monitor_size = primary_monitor.size();

    let window = Arc::new(WindowBuilder::new()
        .with_title("Asteroids Retro Screensaver")
        .with_inner_size(monitor_size)
        .with_fullscreen(None) // Can be set to Some(Fullscreen::Borderless(None)) for fullscreen
        .build(&event_loop)
        .unwrap());

    let mut screensaver = pollster::block_on(AsteroidsScreensaver::new(&window));
    let mut last_frame = Instant::now();
    let window_clone = Arc::clone(&window);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window_clone.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                            ..
                        },
                    ..
                } => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    screensaver.resize(*physical_size);
                }
                WindowEvent::RedrawRequested => {
                    let now = Instant::now();
                    let delta_time = (now - last_frame).as_secs_f32();
                    last_frame = now;

                    screensaver.update(delta_time);

                    match screensaver.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => screensaver.resize(window_clone.inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }

                    window_clone.request_redraw();
                }
                _ => {}
            },
            Event::AboutToWait => {
                window_clone.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
