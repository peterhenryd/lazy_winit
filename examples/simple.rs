#![feature(duration_constants)]

use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};
use lazy_winit::{ApplicationInit, EventLoopExt};

fn main() -> Result<(), EventLoopError> {
    EventLoop::new()?
        .run_lazy_app::<App>(AppOptions {
            title: "Hello, world!".to_owned(),
        })
}

struct AppOptions {
    title: String,
}

impl ApplicationInit for App {
    type Args = AppOptions;

    fn new(event_loop: &ActiveEventLoop, AppOptions { title }: AppOptions) -> Self {
        let attributes: WindowAttributes = WindowAttributes::default()
            .with_title(title);
        let window: Window = event_loop.create_window(attributes).unwrap();

        Self { window, instant: Instant::now(), i: 0 }
    }
}

struct App {
    window: Window,
    instant: Instant,
    i: usize,
}

impl App {
    const TITLES: &'_ [&'_ str] = &["This", "Title", "Is", "Changing"];

    fn get_next_title(&mut self) -> &'static str {
        self.i += 1;
        if self.i >= Self::TITLES.len() {
            self.i = 0;
        }

        Self::TITLES[self.i]
    }
}

impl ApplicationHandler<()> for App {
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
            WindowEvent::RedrawRequested => {
                if self.instant.elapsed() > Duration::SECOND / 2 {
                    self.instant = Instant::now();
                    let title = self.get_next_title();
                    self.window.set_title(title);
                }

                self.window.request_redraw();
            }
            _ => {}
        }
    }
}