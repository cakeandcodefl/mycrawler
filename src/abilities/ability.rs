use bevy::prelude::*;

#[derive(Component)]
pub struct Abilities {}

#[derive(Component)]
pub struct ActiveAbilities {
    pub ability_top_left: Ability,
    pub ability_top_right: Ability,
    pub ability_bottom_left: Ability,
    pub ability_bottom_right: Ability,
}

impl ActiveAbilities {
    pub fn new() -> ActiveAbilities {
        ActiveAbilities {
            ability_top_left: Ability::new(),
            ability_top_right: Ability::new(),
            ability_bottom_left: Ability::new(),
            ability_bottom_right: Ability::new(),
        }
    }
}

#[derive(Component)]
pub struct Ability;

impl Ability {
    pub fn new() -> Ability {
        Ability {}
    }
}
