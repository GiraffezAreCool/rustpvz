use crate::app::engine::rendering_context::{
    PhysicalDevice, QueueFamilies, RenderingContext, RenderingContextAttributes,
};
use anyhow::Result;
use ash::vk;
use std::sync::Arc;
use winit::window::Window;

pub struct Renderer {
    context: Arc<RenderingContext>,
}

impl Renderer {
    pub fn new(context: Arc<RenderingContext>, window: Arc<Window>) -> Result<Self> {
        Ok(Self { context })
    }
}
