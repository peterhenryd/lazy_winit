use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event_loop::EventLoop;
use crate::app::App;
use crate::application_init::ApplicationInit;
use crate::unsafe_app::UnsafeApp;

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

pub trait EventLoopUnitExit {
    fn run_lazy<T: ApplicationInit + ApplicationHandler<()> + 'static>(self) -> Result<(), EventLoopError>;
}