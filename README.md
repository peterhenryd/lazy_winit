# lazy_winit

A simple library for initializing [`winit`](https://github.com/rust-windowing/winit) application state lazily without dealing with nullables.

```rust
fn main() -> Result<(), EventLoopError> {
    use lazy_winit::EventLoopExt;
    
    // This will run your application lazy, and will not check if it is initialized before being called. This may be
    // helpful for performance reasons, but may not be desirable on certain platforms.
    EventLoop::new()?.run_lazy_app_unchecked::<App, _>("Window Title".to_owned());
        
    // This will run your application lazily, and ensure that is initialized before being called.
    EventLoop::new()?.run_lazy_app::<App, _>("Window Title".to_owned());
}

struct App { window: Window }

impl lazy_winit::ApplicationInit for App {
    type State = String;
    
    fn new(event_loop: &ActiveEventLoop, title: Self::State) -> Self {
        let attributes: WindowAttributes = WindowAttributes::default()
            .with_title(title);
        let window: Window = event_loop.create_window(attributes).unwrap();

        Self { window }
    }
}

impl ApplicationHandler for App {
    // ...
}
```