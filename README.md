# lazy_winit

A simple library for initializing [`winit`](https://github.com/rust-windowing/winit) application state lazily without dealing with nullables.

| Compatibility | `winit` | `lazy_winit` |
|---------------|---------|--------------|
| Version       | 0.30.8  | 0.1.1        |

## How to install

From the terminal:
```
cargo add lazy_winit
```

In the `[dependencies]` section of your Cargo.toml:
```
lazy_winit = 0.1.1
```

## Example

```rust
fn main() -> Result<(), EventLoopError> {
    use lazy_winit::EventLoopExt;
    
    // This will run your application lazily, and will not ensure that it is initialized before being called.
    // This may be helpful for performance reasons, but may not be desirable on certain platforms.
    EventLoop::new()?.run_lazy_app_unchecked::<App>("Window Title".to_owned());
        
    // This will run your application lazily, and will ensure that it is initialized before being called.
    EventLoop::new()?.run_lazy_app::<App>("Window Title".to_owned());
}

struct App { window: Window }

impl lazy_winit::ApplicationInit for App {
    type Args = String;
    
    fn new(event_loop: &ActiveEventLoop, title: Self::Args) -> Self {
        let attributes = WindowAttributes::default().with_title(title);
        let window: Window = event_loop.create_window(attributes).unwrap();

        Self { window }
    }
}

impl ApplicationHandler for App {
    // ...
}
```
