use winit::event_loop::ActiveEventLoop;

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