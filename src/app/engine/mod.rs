mod renderer;
mod rendering_context;
use crate::app::engine::rendering_context::queue_family_picker;
use crate::app::engine::rendering_context::{
    PhysicalDevice, QueueFamilies, RenderingContext, RenderingContextAttributes,
};
use anyhow::{Ok, Result};
use std::{collections::HashMap, sync::Arc};
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::app::engine::renderer::Renderer;

pub struct Engine {
    renderers: HashMap<WindowId, Renderer>,
    windows: HashMap<WindowId, Arc<Window>>,
    rendering_context: Arc<RenderingContext>,
    primary_window_Id: WindowId,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        let primary_window = Arc::new(event_loop.create_window(Default::default())?);
        let primary_window_Id = primary_window.id();

        let rendering_context = Arc::new(RenderingContext::new(RenderingContextAttributes {
            compatibility_window: primary_window.as_ref(),
            queue_family_picker: queue_family_picker::single_queue_family,
        })?);

        let windows = HashMap::from([(primary_window_Id, primary_window)]);
        let renderers = windows
            .iter()
            .map(|(id, window)| {
                let renderer = Renderer::new(rendering_context.clone(), window.clone()).unwrap();
                (*id, renderer)
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            renderers,
            windows,
            rendering_context,
            primary_window_Id,
        })
    }

    pub fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if window_id == self.primary_window_Id {
                    event_loop.exit();
                } else {
                    self.windows.remove(&window_id);
                    self.renderers.remove(&window_id);
                }
            }
            _ => {}
        }
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
    ) -> Result<WindowId> {
        let window = Arc::new(event_loop.create_window(attributes)?);
        let window_id = window.id();
        self.windows.insert(window_id, window.clone());

        let renderer = Renderer::new(self.rendering_context.clone(), window)?;
        self.renderers.insert(window_id, renderer);

        Ok(window_id)
    }
}
