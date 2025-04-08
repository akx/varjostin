use egui::TextureHandle;

#[derive(Clone, Default)]
pub struct WrappedTexture {
    pub handle: Option<TextureHandle>,
}

pub type Textures = [WrappedTexture; 4];
