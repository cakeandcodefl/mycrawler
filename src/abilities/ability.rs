use bevy::prelude::*;
use std::sync::Arc;
use std::ops::Deref;

#[derive(Component)]
pub struct Abilities {}

#[derive(Component, Clone)]
pub struct ActiveAbilities {
    pub active_abilities: Vec<Arc<dyn Ability + Send + Sync>>, //TODO: think if you need to box this into a Mutex instead of a Arc
}

impl ActiveAbilities {
    pub fn new() -> ActiveAbilities {
        ActiveAbilities {
            active_abilities: vec![],
        }
    }

    pub fn add_ability(&mut self, ability: Arc<dyn Ability + Send + Sync>) {
        //TODO: make sure that we keep the size <10 
        self.active_abilities.push(ability);
    }

    pub fn remove_ability(&mut self, ability_to_remove: Arc<dyn Ability + Send + Sync>) {
        //TODO: make sure that we keep the size <10 
        self.active_abilities.retain(|ability| ability.deref().get_ability_name() != ability_to_remove.deref().get_ability_name());
    }
    //TODO:create update fn
}

pub trait Ability {
    fn get_ability_name(&self) -> String;
} 