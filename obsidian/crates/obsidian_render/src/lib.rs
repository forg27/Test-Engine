pub mod render;
pub use crate::render::{Backend, Render};
pub enum Backend {
    Vulkan,
}
pub trait Render {
    fn render(
        &mut self,
        dimensions: &[u32; 2],
    ) -> Result<()>;
}
