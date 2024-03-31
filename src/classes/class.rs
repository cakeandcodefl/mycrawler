use crate::stats::base_stats::*;
use bevy::prelude::*;
use std::str::FromStr;

#[derive(Component)]
pub struct Character {
    pub character_type: CharacterType,
    pub level: u32,
    pub health: Health,
    pub defense: Defense,
    pub phys_atk: PhysAtk,
    pub magic_atk: MagicAtk,
}

//TODO: think about if def/atk values should be in the class or the underlying class type
impl Character {
    pub fn new(character_type: CharacterType) -> Character {
        //TODO:change this depending on selected class
        Character {
            character_type: character_type,
            level: 1,
            health: Health { hp_value: 100 },
            defense: Defense { def_value: 10 },
            phys_atk: PhysAtk { phys_atk_value: 1 },
            magic_atk: MagicAtk { magic_atk_value: 1 },
        }
    }

    //TODO:create a update method to update stats on actions
}

impl FromStr for Character {
    type Err = ();

    fn from_str(input: &str) -> Result<Character, Self::Err> {
        match input {
            "Mage" => Ok(Character::new(CharacterType::Mage)),
            "Hunter" => Ok(Character::new(CharacterType::Hunter)),
            "Warrior" => Ok(Character::new(CharacterType::Warrior)),
            "Priest" => Ok(Character::new(CharacterType::Priest)),
            "Rogue" => Ok(Character::new(CharacterType::Rogue)),
            _ => Err(()),
        }
    }
}

#[derive(Component)]
pub enum CharacterType {
    Warrior,
    Mage,
    Rogue,
    Hunter,
    Priest,
    NonCharacter, //TODO: remove this later on
}
