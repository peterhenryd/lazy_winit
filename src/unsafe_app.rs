use crate::application_init::ApplicationInit;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

/// A wrapper for an optional of an application initializer and handler.
///
/// This wrapper, when executed, will not check if the application is initialized before calling any
/// of its methods.
pub struct UnsafeApp<T, S>(Option<T>, Option<S>);

impl<E: 'static, T: ApplicationInit + ApplicationHandler<E> + 'static> ApplicationHandler<E> for UnsafeApp<T, T::Args> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if let Some(handler) = &mut self.0 {
            handler.new_events(event_loop, cause);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut app = T::new(event_loop, self.1.take().unwrap());
        app.resumed(event_loop);
        self.0 = Some(app);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: E) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.user_event(event_loop, event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.window_event(event_loop, window_id, event);
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.device_event(event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.about_to_wait(event_loop);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.suspended(event_loop);
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.exiting(event_loop);
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.memory_warning(event_loop);
    }
}

impl<T: ApplicationInit> UnsafeApp<T, T::Args> {
    pub fn new(args: T::Args) -> Self {
        Self(None, Some(args))
    }
}