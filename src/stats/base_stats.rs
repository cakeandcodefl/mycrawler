use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Health {
    pub hp_value: u32,
}

#[derive(Component, Clone)]
pub struct Defense {
    pub def_value: u32,
}

#[derive(Component, Clone)]
pub struct PhysAtk {
    pub phys_atk_value: u32,
}

#[derive(Component, Clone)]
pub struct MagicAtk {
    pub magic_atk_value: u32,
}
