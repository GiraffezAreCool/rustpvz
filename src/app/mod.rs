mod engine;

use crate::app::engine::Engine;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

#[derive(Default)]
pub struct App {
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop).unwrap());
        if let Some(engine) = self.engine.as_mut() {
            let secondary_window = engine
                .create_window(
                    event_loop,
                    WindowAttributes::default().with_title("Second Window"),
                )
                .unwrap();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let (Some(engine)) = self.engine.as_mut() {
            engine.window_event(event_loop, window_id, event);
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = None;
    }
}
