use crate::Render;
use anyhow::Result;
use raw_window_handle::HasRawWindowHandle;
use log::info;
pub(crate) use self::render::VulkanRenderBackend;

pub(crate) struct VulkanRenderBackend;

impl Render for VulkanRenderBackend{
    fn render(&mut self, _dimensions: &[u32; 2],
    ) ->Result<()> {
        Ok(())
    }
}