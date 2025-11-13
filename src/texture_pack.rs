use crate::texture::{Texture, TextureWithNormal};
use std::collections::HashMap;

pub struct TexturePack {
    textures: Vec<TextureWithNormal>,
    name_to_id: HashMap<String, usize>,
}

impl TexturePack {
    pub fn new() -> Self {
        TexturePack {
            textures: Vec::new(),
            name_to_id: HashMap::new(),
        }
    }

    /// Register a texture with optional normal and specular maps
    pub fn add_texture(
        &mut self,
        name: &str,
        diffuse_path: &str,
        normal_path: Option<&str>,
        specular_path: Option<&str>,
    ) -> usize {
        let texture = TextureWithNormal::load(diffuse_path, normal_path, specular_path);
        let id = self.textures.len();
        self.textures.push(texture);
        self.name_to_id.insert(name.to_string(), id);

        println!("Registered texture '{}' with ID {}", name, id);
        id
    }

    /// Get texture by ID
    pub fn get_texture(&self, id: usize) -> Option<&TextureWithNormal> {
        self.textures.get(id)
    }

    /// Get texture ID by name
    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.name_to_id.get(name).copied()
    }

    /// Get number of textures
    pub fn len(&self) -> usize {
        self.textures.len()
    }
}

/// Global texture pack manager
pub struct TextureManager {
    packs: Vec<TexturePack>,
    active_pack: usize,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            packs: Vec::new(),
            active_pack: 0,
        }
    }

    /// Add a texture pack
    pub fn add_pack(&mut self, pack: TexturePack) {
        self.packs.push(pack);
    }

    /// Switch to next texture pack (cycles)
    pub fn next_pack(&mut self) {
        if !self.packs.is_empty() {
            self.active_pack = (self.active_pack + 1) % self.packs.len();
            println!("Switched to texture pack {}", self.active_pack);
        }
    }

    /// Get active texture pack
    pub fn active_pack(&self) -> Option<&TexturePack> {
        self.packs.get(self.active_pack)
    }

    /// Get number of packs
    pub fn num_packs(&self) -> usize {
        self.packs.len()
    }
}
