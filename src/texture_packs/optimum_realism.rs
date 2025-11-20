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
        None,
    );
    pack.add_texture(
        "glass",
        &format!("{}/glass.png", base_path),
        Some(&format!("{}/glass_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "gold",
        &format!("{}/gold_block.png", base_path),
        Some(&format!("{}/gold_block_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "dirt",
        &format!("{}/dirt.png", base_path),
        Some(&format!("{}/dirt_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "grass_top",
        &format!("{}/grass_block_top.png", base_path),
        Some(&format!("{}/grass_block_top_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "stone",
        &format!("{}/stone.png", base_path),
        Some(&format!("{}/stone_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "glowstone",
        &format!("{}/glowstone.png", base_path),
        Some(&format!("{}/glowstone_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "oak_leaves",
        &format!("{}/oak_leaves.png", base_path),
        Some(&format!("{}/oak_leaves_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "emerald_ore",
        &format!("{}/emerald_ore.png", base_path),
        Some(&format!("{}/emerald_ore_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "honeycomb",
        &format!("{}/honeycomb_block.png", base_path),
        Some(&format!("{}/honeycomb_block_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "ice",
        &format!("{}/ice.png", base_path),
        Some(&format!("{}/ice_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "iron_ore",
        &format!("{}/iron_ore.png", base_path),
        Some(&format!("{}/iron_ore_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "stripped_oak_log",
        &format!("{}/stripped_oak_log.png", base_path),
        Some(&format!("{}/stripped_oak_log_n.png", base_path)),
        None,
    );
    pack.add_texture(
        "tnt",
        &format!("{}/tnt_side.png", base_path),
        Some(&format!("{}/tnt_side_n.png", base_path)),
        None,
    );

    pack
}

pub mod texture_ids {
    pub const OAK_LOG: usize = 0;
    pub const DIAMONG_BLOCK: usize = 1;
    pub const GLASS: usize = 2;
    pub const GOLD: usize = 3;
    pub const DIRT: usize = 4;
    pub const GRASS_TOP: usize = 5;
    pub const STONE: usize = 6;
    pub const GLOWSTONE: usize = 7;
    pub const OAK_LEAVES: usize = 8;
    pub const EMERALD_ORE: usize = 9;
    pub const HONEYCOMB: usize = 10;
    pub const ICE: usize = 11;
    pub const IRON_ORE: usize = 12;
    pub const STRIPPED_OAK_LOG: usize = 13;
    pub const TNT: usize = 14;
}
