use mimalloc::MiMalloc;
use winit::event::{Event, WindowEvent};

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Test Application")
        .with_theme(Some(winit::window::Theme::Dark))
        .build(&event_loop)
        .expect("Failed to create an instance of Window.");

    let _engine = engine::Engine::new(&window).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            _ => (),
        },
        _ => (),
    });
}
