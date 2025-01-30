use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

/// An initializer for a [winit] application. This trait's function is called during
/// the outer application's [ApplicationHandler::resumed] to initialize the application.
///
/// Types implementing this trait also generally implement [ApplicationHandler]. For such types,
/// their [ApplicationHandler::resumed] implementation is called directly after
/// [ApplicationInit::new].
///
/// In order to run the application, you can use wrap your type into [App], and call
/// [EventLoop::run_app], or use [EventLoopExt::run_lazy_app].
pub trait ApplicationInit {
    /// The arguments used to create the application.
    type Args;

    fn new(event_loop: &ActiveEventLoop, args: Self::Args) -> Self;
}

/// A wrapper for an optional of an application initializer and handler.
///
/// This wrapper, when executed, will ensure the application is initialized before calling any of
/// its methods.
pub struct App<T, A>(Option<T>, Option<A>);

impl<E: 'static, T: ApplicationInit + ApplicationHandler<E> + 'static> ApplicationHandler<E> for App<T, T::Args> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if let Some(handler) = &mut self.0 {
            handler.new_events(event_loop, cause);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.0 = Some(T::new(event_loop, self.1.take().unwrap()));
        self.0.as_mut().unwrap().resumed(event_loop);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: E) {
        if let Some(handler) = &mut self.0 {
            handler.user_event(event_loop, event);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if let Some(handler) = &mut self.0 {
            handler.window_event(event_loop, window_id, event);
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        if let Some(handler) = &mut self.0 {
            handler.device_event(event_loop, device_id, event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(handler) = &mut self.0 {
            handler.about_to_wait(event_loop);
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(handler) = &mut self.0 {
            handler.suspended(event_loop);
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(handler) = &mut self.0 {
            handler.exiting(event_loop);
        }
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(handler) = &mut self.0 {
            handler.memory_warning(event_loop);
        }
    }
}

impl<T: ApplicationInit> App<T, T::Args> {
    pub fn new(args: T::Args) -> Self {
        Self(None, Some(args))
    }
}

/// A wrapper for an optional of an application initializer and handler.
///
/// This wrapper, when executed, will not check if the application is initialized before calling any
/// of its methods.
pub struct UnsafeApp<T, S>(Option<T>, Option<S>);

impl<E: 'static, T: ApplicationInit + ApplicationHandler<E> + 'static> ApplicationHandler<E> for UnsafeApp<T, T::Args> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        unsafe { self.0.as_mut().unwrap_unchecked() }.new_events(event_loop, cause);
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

pub trait EventLoopExt<E: 'static> {
    fn run_lazy_app<T: ApplicationInit + ApplicationHandler<E> + 'static>(self, args: T::Args) -> Result<(), EventLoopError>;

    fn run_lazy_app_unchecked<T: ApplicationInit + ApplicationHandler<E> + 'static>(self, args: T::Args) -> Result<(), EventLoopError>;
}

impl<E: 'static> EventLoopExt<E> for EventLoop<E> {
    fn run_lazy_app<T: ApplicationInit + ApplicationHandler<E> + 'static>(self, args: T::Args) -> Result<(), EventLoopError> {
        self.run_app(&mut App::<T, T::Args>::new(args))
    }

    fn run_lazy_app_unchecked<T: ApplicationInit + ApplicationHandler<E> + 'static>(self, args: T::Args) -> Result<(), EventLoopError> {
        self.run_app(&mut UnsafeApp::<T, T::Args>::new(args))
    }
}