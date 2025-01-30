use lazy_winit::{ApplicationInit, EventLoopExt};
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

fn main() -> Result<(), EventLoopError> {
    let mut event_loop = EventLoop::new()?;

    event_loop.run_lazy_app::<App>("Window Example".to_owned())
}

struct App {
    window: Window,
}

impl ApplicationInit for App {
    type Args = String;

    fn new(event_loop: &ActiveEventLoop, title: String) -> Self {
        let attributes: WindowAttributes = WindowAttributes::default()
            .with_title(title);
        let window: Window = event_loop.create_window(attributes).unwrap();

        Self { window }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else { return };

                if code == KeyCode::Escape {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}