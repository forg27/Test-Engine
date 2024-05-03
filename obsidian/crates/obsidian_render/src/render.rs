use raw_window_handle::HasRawWindowHandle;
use anyhow::Result;

pub enum Backend {
    Vulkan,
}
pub trait Render {
    fn render(
        &mut self,
        dimensions: &[u32; 2],
    ) -> Result<()>;
}



#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanRenderBackend;

impl dyn Render {
    pub fn create_backend(
        backend: &Backend,
        window_handle: &impl HasRawWindowHandle,
        dimensions: &[u32; 2],
    ) -> Result<impl Render> {
        match backend {
            Backend::Vulkan => VulkanRenderBackend::new(window_handle, dimensions),
        }
    }
}
