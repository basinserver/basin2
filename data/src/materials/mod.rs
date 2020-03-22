use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub enum PushReaction {
    Normal,
    Destroy,
    Block,
    Ignore,
    PushOnly,
}

#[derive(Debug)]
pub struct MaterialT {
    push_reaction: PushReaction,
    blocks_motion: bool,
    flammable: bool,
    always_destroyable: bool,
    liquid: bool,
    solid_blocking: bool,
    replaceable: bool,
    solid: bool,
}

impl Default for MaterialT {
    fn default() -> MaterialT {
        MaterialT {
            push_reaction: PushReaction::Normal,
            blocks_motion: true,
            flammable: false,
            always_destroyable: true,
            liquid: false,
            solid_blocking: true,
            replaceable: false,
            solid: true,
        }
    }
}

pub type Material = Arc<MaterialT>;

lazy_static! {
    pub static ref AIR: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            replaceable: false,
            ..Default::default()
        })
    };
    pub static ref STRUCTURAL_AIR: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            replaceable: false,
            ..Default::default()
        })
    };
    pub static ref PORTAL: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Block,
            ..Default::default()
        })
    };
    pub static ref CLOTH_DECORATION: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            flammable: true,
            ..Default::default()
        })
    };
    pub static ref PLANT: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref WATER_PLANT: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref REPLACEABLE_PLANT: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            flammable: true,
            ..Default::default()
        })
    };
    pub static ref REPLACEABLE_WATER_PLANT: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            ..Default::default()
        })
    };
    pub static ref WATER: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            liquid: true,
            ..Default::default()
        })
    };
    pub static ref BUBBLE_COLUMN: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            liquid: true,
            ..Default::default()
        })
    };
    pub static ref LAVA: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            liquid: true,
            ..Default::default()
        })
    };
    pub static ref TOP_SNOW: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            always_destroyable: false,
            ..Default::default()
        })
    };
    pub static ref FIRE: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            replaceable: false,
            ..Default::default()
        })
    };
    pub static ref DECORATION: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            solid: false,
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref WEB: Material = {
        Arc::new(MaterialT {
            blocks_motion: false,
            solid_blocking: false,
            push_reaction: PushReaction::Destroy,
            always_destroyable: false,
            ..Default::default()
        })
    };
    pub static ref BUILDABLE_GLASS: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref CLAY: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref DIRT: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref GRASS: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref ICE_SOLID: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref SAND: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref SPONGE: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref SHULKER_SHELL: Material = {
        Arc::new(MaterialT {
            ..Default::default()
        })
    };
    pub static ref WOOD: Material = {
        Arc::new(MaterialT {
            flammable: true,
            ..Default::default()
        })
    };
    pub static ref BAMBOO_SAPLING: Material = {
        Arc::new(MaterialT {
            flammable: true,
            push_reaction: PushReaction::Destroy,
            blocks_motion: false,
            ..Default::default()
        })
    };
    pub static ref BAMBOO: Material = {
        Arc::new(MaterialT {
            flammable: true,
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref WOOL: Material = {
        Arc::new(MaterialT {
            flammable: true,
            ..Default::default()
        })
    };
    pub static ref EXPLOSIVE: Material = {
        Arc::new(MaterialT {
            flammable: true,
            solid_blocking: false,
            ..Default::default()
        })
    };
    pub static ref LEAVES: Material = {
        Arc::new(MaterialT {
            flammable: true,
            solid_blocking: false,
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref GLASS: Material = {
        Arc::new(MaterialT {
            solid_blocking: false,
            ..Default::default()
        })
    };
    pub static ref ICE: Material = {
        Arc::new(MaterialT {
            solid_blocking: false,
            ..Default::default()
        })
    };
    pub static ref CACTUS: Material = {
        Arc::new(MaterialT {
            solid_blocking: false,
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref STONE: Material = {
        Arc::new(MaterialT {
            always_destroyable: false,
            ..Default::default()
        })
    };
    pub static ref METAL: Material = {
        Arc::new(MaterialT {
            always_destroyable: false,
            ..Default::default()
        })
    };
    pub static ref SNOW: Material = {
        Arc::new(MaterialT {
            always_destroyable: false,
            ..Default::default()
        })
    };
    pub static ref HEAVY_METAL: Material = {
        Arc::new(MaterialT {
            always_destroyable: false,
            push_reaction: PushReaction::Block,
            ..Default::default()
        })
    };
    pub static ref BARRIER: Material = {
        Arc::new(MaterialT {
            always_destroyable: false,
            push_reaction: PushReaction::Block,
            ..Default::default()
        })
    };
    pub static ref PISTON: Material = {
        Arc::new(MaterialT {
            push_reaction: PushReaction::Block,
            ..Default::default()
        })
    };
    pub static ref CORAL: Material = {
        Arc::new(MaterialT {
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref VEGETABLE: Material = {
        Arc::new(MaterialT {
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref EGG: Material = {
        Arc::new(MaterialT {
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
    pub static ref CAKE: Material = {
        Arc::new(MaterialT {
            push_reaction: PushReaction::Destroy,
            ..Default::default()
        })
    };
}
