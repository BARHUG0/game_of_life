use crate::texture_pack::TexturePack;

/// Load the default texture pack
pub fn load_default_pack() -> TexturePack {
    let mut pack = TexturePack::new();

    // Define base path for this pack
    let base_path = "assets/texture_packs/optimum_realism";

    // Register textures with their optional normal maps
    // Format: add_texture(name, diffuse_path, optional_normal_path)

    pack.add_texture(
        "oak_log",
        &format!("{}/oak_log.png", base_path),
        Some(&format!("{}/oak_log_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "diamond_block",
        &format!("{}/diamond_block.png", base_path),
        Some(&format!("{}/diamond_block_n.png", base_path)),
        Some(&format!("{}/diamond_block_s.png", base_path)),
    );
    pack.add_texture(
        "glass",
        &format!("{}/glass.png", base_path),
        Some(&format!("{}/glass_n.png", base_path)),
        Some(&format!("{}/glass_s.png", base_path)),
    );
    pack.add_texture(
        "gold",
        &format!("{}/gold_block.png", base_path),
        Some(&format!("{}/gold_block_n.png", base_path)),
        Some(&format!("{}/gold_block_s.png", base_path)),
    );

    pack
}

// Texture ID constants for easy reference
pub mod texture_ids {
    pub const OAK_LOG: usize = 0;
    pub const DIAMONG_BLOCK: usize = 1;
    pub const GLASS: usize = 2;
    pub const GOLD: usize = 3;
}
