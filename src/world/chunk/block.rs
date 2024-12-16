#[derive(Clone, Copy)]
pub enum BlockLayer {
    Background,
    Foreground,
}

#[derive(Clone, Copy)]
pub struct Block<'a> {
    pub id: u32,
    texture_name: &'a str,
    layer: BlockLayer,
    is_solid: bool,
    durability: u8,
}

impl Block<'_>{
    pub fn new(id: u32) -> Self {
        let is_solid = if id == 0 {
            false
        } else { true };

        Self {
            id,
            texture_name: "dirt.png",
            layer: BlockLayer::Foreground,
            is_solid,
            durability: 2,
        }
    }
}